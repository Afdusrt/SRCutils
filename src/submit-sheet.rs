use std::{ env, fs, io, process, process::Command, time::{Duration, SystemTime}, thread::sleep, };

fn parse_to_seconds(input: &str) -> String {
    let parts: Vec<&str> = input.split(':').collect();

    let mut h = 0.0;
    let mut m = 0.0;
    let mut s = 0.0;

    match parts.len() {
        1 => {
            s = parts[0].parse().expect("not a valid float");
        }
        2 => {
            m = parts[0].parse().expect("not a valid float");
            s = parts[1].parse().expect("not a valid float");
        }
        3 => {
            h = parts[0].parse().expect("not a valid float");
            m = parts[1].parse().expect("not a valid float");
            s = parts[2].parse().expect("not a valid float");
        }
        _ => {
            eprintln!("INVALID TEXT FILE FORMAT, wrong time");
            std::process::exit(5)
        }
    }

    let s = (s * 1000.0 + m * 60_000.0 + h * 3_600_000.0) / 1000.0;
    s.to_string()
}
fn fetch(s: &str, n: &str) -> String {
	Command::new("curl")
		.arg("-L")
		.arg(format!("http://www.speedrun.com/api/{}", s))
		.arg("-o")
		.arg(n)
		.status()
	.expect("Failure");
	
	let raw_json = fs::read_to_string(n)
		.expect("Didnt read file");
	
	//let json = decode_weird_unicode(&raw_json);
	
	fs::write(n, &raw_json).expect("Could not write file");
	
	return raw_json
}
#[derive(Debug)]
struct Variable {
	id: String,
	name: String,
	if_il_id: String,
	options: Vec<String>,
}
const HELP: &str = "HELP:
====
arg 1 - api key, from speedrun.com -> settings -> api key
arg 2 - csv file to submit
=
You will be prompted to validate that the runs that will be submitted are correct.
You will need to open the csv file in a program like libreoffice, to fill out correct values before using submit-sheet, refer to the README.
Ensure the last line of the csv file is in correspondce to the spec.
";

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() < 3 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
    let api_key = &args[1];
    let file_path = &args[2];

    let f = fs::read_to_string(file_path).unwrap();
    
    let lines: Vec<&str> = f.lines().collect(); 

    let mut matrix: Vec<Vec<&str>> = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i].split('|').collect();
        matrix.push(line);
    }

    println!("{:?}", matrix[3][0]);

    let mut first_run_line_index: i32 = matrix[3][0].parse().expect("");
    first_run_line_index -= 1;
    println!("{:?}", matrix[first_run_line_index as usize]);
	//let variables_raw = fetch("v1/games/nd28p43d/variables", "variables2");
	let variables_raw = fs::read_to_string("variables2").unwrap();
    let parsed_variables = json::parse(&variables_raw).unwrap();
    //let mut vars: Vec<Variable> = Vec::new();
    let mut vars_per_level: Vec<Variable> = Vec::new();
    let mut vars: Vec<Variable> = Vec::new();
    for i in 0..parsed_variables["data"].len() {
		//variable==== id/ name
		println!("{}/ {}", parsed_variables["data"][i]["id"], parsed_variables["data"][i]["name"]);
		
		let mut is_per_level = false;
		let mut if_il_id = String::new();
		
		if parsed_variables["data"][i]["scope"]["type"] == "single-level" {
			if_il_id = parsed_variables["data"][i]["scope"]["level"].to_string();
			is_per_level = true;
		}
		
		let mut options: Vec<String> = Vec::new();
		//varible options
		for (id, name) in parsed_variables["data"][i]["values"]["values"].entries() {
			let option = format!("{}/ {}", id, name["label"]);
			//println!("===={}/ {}", id, name["label"]);
			options.push(option);
		}
		
		//variable==== options Vec<optionID,optionNAME
		//println!("{:?}", options);
		if is_per_level {
			vars_per_level.push( Variable {
					id: parsed_variables["data"][i]["id"].to_string(),
					name: parsed_variables["data"][i]["name"].to_string(),
					if_il_id: if_il_id,
					options: options, //Vec<String>
			});
		} else {
			vars.push( Variable {
					id: parsed_variables["data"][i]["id"].to_string(),
					name: parsed_variables["data"][i]["name"].to_string(),
					if_il_id: if_il_id,
					options: options, //Vec<String>
			});
		}
		//println!("{}/ {} |Options:", vars[i].id, vars[i].name);
	}
	
    let mut runs: Vec<String> = Vec::new();
    let mut ppp_runs: Vec<String> = Vec::new();

    let mut payload = String::new();
    payload.push_str("{\"run\": {");

    let cat_id = &matrix[1][0][0..8];
    for i in first_run_line_index as usize..matrix.len() { 
        let mut payload = String::new();
        payload.push_str("{\"run\": {\n");
        payload.push_str(&format!(" \"category\": \"{}\",\n", cat_id));
        
        if matrix[2][0] == "true" {
            payload.push_str(&format!(" \"level\": \"{}\",\n", &matrix[i][5][0..8]));
            //println!("levels");
        }

        if matrix[i][0] != "NO" {
            payload.push_str(&format!(" \"date\": \"{}\",\n", &matrix[i][0]));
        }

        if matrix[i][1] != "NO" {
            payload.push_str(&format!(" \"region\": \"{}\",\n", &matrix[i][1][0..8]));
        }

        if matrix[i][2] != "NO" {
            payload.push_str(&format!(" \"platform\": \"{}\",\n", &matrix[i][2][0..8]));
        }

        if matrix[i][3] != "NO" {
            payload.push_str(" \"emulated\": true,\n",);
        } else {
            payload.push_str(" \"emulated\": false,\n",);
        }

        //if matrix[i][4] != "NO" ----players will not implement until coop submissions work for moderators again
		
		//times
        payload.push_str(" \"times\": {\n");
        let mut should_next_comma_left = false;
		if matrix[i][6] != "NO" { //LRT
			payload.push_str(&format!("  \"realtime_noloads\": {}", parse_to_seconds(&matrix[i][6]) ) );
			should_next_comma_left = true;
		}
		if matrix[i][7] != "NO" { //RTA
			if should_next_comma_left {
				payload.push_str(",\n");
			}
			payload.push_str(&format!("  \"realtime\": {}", parse_to_seconds(&matrix[i][7]) ) );
			should_next_comma_left = true;
		}
		if matrix[i][8] != "NO" { //IGT
			if should_next_comma_left {
				payload.push_str(",\n");
			}
			payload.push_str(&format!("  \"ingame\": {}", parse_to_seconds(&matrix[i][8]) ) );
			should_next_comma_left = true;
		}
		payload.push_str("\n },\n");
		
		payload.push_str(&format!(" \"video\": \"{}\",\n", &matrix[i][9]));
		payload.push_str(&format!(" \"comment\": \"{}\",\n", &matrix[i][10]));
		
		//VARIABLES
		should_next_comma_left = false;
		payload.push_str(" \"variables\": {\n");
		
		//println!("Total length: {}", matrix[i].len());
		//first var is at 11
		for n in ( 11..matrix[i].len() ).step_by(2) {
			if n + 1 >= matrix[i].len() {
				break;
			}
			if matrix[i][n].is_empty() || matrix[i][n+1].is_empty() {
				continue;
			}
			if should_next_comma_left {
				payload.push_str(",\n");
			}
			payload.push_str(&format!(
			"  \"{}\": {{ \"type\": \"pre-defined\", \"value\": \"{}\" }}", &matrix[i][n][0..8], &matrix[i][n+1][0..8])
			);
			should_next_comma_left = true;
			//print!("Variable: {}, option: {}\n", &matrix[i][n], &matrix[i][n+1]);
		}
		//should_next_comma_left = false;
		
		let pre_patch_payload = payload.clone();
			//======= //shit
			for var in vars_per_level.iter() {
				let var_id = &var.id[0..8];
				
				// check if this variable is already in the payload
				if payload.contains(&format!("\"{}\":", var_id)) {
					continue;
				}
				
				let default_value = &var.options[0][..8];
				
				if should_next_comma_left {
					payload.push_str(",\n");
				}
				payload.push_str(&format!(
					//", \"{}\": {{ \"type\": \"pre-defined\", \"value\": \"{}\" }}",
					"  \"{}\": {{ \"type\": \"pre-defined\", \"value\": \"{}\" }}",
					var_id, default_value
				));
				
				should_next_comma_left = true;
			}
			
			for var in vars.iter() {
				let var_id = &var.id[0..8];

				if payload.contains(&format!("\"{}\":", var_id)) {
					continue;
				}

				let default_value = &var.options[0][..8];
				
				if should_next_comma_left {
					payload.push_str(",\n");
				}
				payload.push_str(&format!(
					"  \"{}\": {{ \"type\": \"pre-defined\", \"value\": \"{}\" }}",
					var_id, default_value
				));
				
				should_next_comma_left = true;
			}
			//=======
		payload.push_str("\n }");// close variables
		payload.push_str("\n}}");
		//==============
        ppp_runs.push(pre_patch_payload.clone());
        runs.push(payload.clone());
     //   println!("{}", payload);
        //println!("{}", matrix[i as usize][5]);
    }
    
    for i in 0..ppp_runs.len() {
		println!("{}", ppp_runs[i]);
	}
//println!("{:?}", vars_per_level);
	println!("==================\nSubmit the speedruns? (yes or no)");
	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let trimmed = input_text.trim().to_lowercase();
    
    if trimmed == "no" {
		for i in 0..runs.len() {
		println!("{}", runs[i]);
	}
		return
	}
	//==================
	
	
	//==================
	for i in 0..runs.len() {
		let output = Command::new("curl")
			.arg("-L")
			.arg("-X")
			.arg("POST")
			.arg("https://www.speedrun.com/api/v1/runs")
			.arg("-H")
			.arg("Content-Type: application/json")
			.arg("-H")
			.arg(format!("X-Api-Key: {}", api_key))
			.arg("--data-raw")
			.arg(runs[i].clone())		
			.output()
		.expect("Failure");
				
		let string_to_print = String::from_utf8_lossy(&output.stdout);
		println!("{:?}", string_to_print);
		println!("Sleeping for 2s");
		sleep(Duration::new(2, 0));
	}
	println!("=========\nDone. Check your pending.");
}
