use std::{fs, io, process, time::{Duration}, thread::sleep, };

fn parse_to_seconds(input: &str) -> String {
    let parts: Vec<&str> = input.split(':').collect();

    let seconds = match parts.len() {
        1 => {
            parts[0].parse::<f64>().expect("not a valid float")
        }
        2 => {
            let m = parts[0].parse::<f64>().expect("not a valid float");
            let s = parts[1].parse::<f64>().expect("not a valid float");

            m * 60.0 + s
        }
        3 => {
            let h = parts[0].parse::<f64>().expect("not a valid float");
            let m = parts[1].parse::<f64>().expect("not a valid float");
            let s = parts[2].parse::<f64>().expect("not a valid float");

            h * 3600.0 + m * 60.0 + s
        }
        _ => {
            eprintln!("INVALID TEXT FILE FORMAT, wrong time");
            std::process::exit(5);
        }
    };

    seconds.to_string()
}

/*fn fetch(s: &str) -> Result<String, ureq::Error> {
	let url = format!(
        "https://www.speedrun.com/api/{}",
        s
    );
	let response = ureq::get(url).call();

    let mut resp_1 = match response {
        //read_to_string needs mut body
        Ok(response_body) => response_body,
        Err(e) => {
            /*eprintln!("Failed fetch: {}", e);*/
            return Err(e);
        }
    };

    match resp_1.body_mut().read_to_string() {
        //read_to_string needs mut body
        Ok(body_string) => return Ok(body_string),
        Err(e) => {
            /*eprintln!("Failed read to string: {}", e);*/
            return Err(e);
        }
    };
}*/
/*
#[derive(Debug)]
struct Variable {
	id: String,
	name: String,
	if_il_id: String,
	options: Vec<String>,
}*/
const HELP: &str = "HELP:
====
arg 2 - api key, from speedrun.com -> settings -> api key
arg 3 - csv file to submit
arg 4 - sleep time in milliseconds between requests
=
You will be prompted to validate that the runs that will be submitted are correct.
You will need to open the csv file in a program like libreoffice, to fill out correct values before using submit-sheet, refer to the README.
Ensure the last line of the csv file is in correspondce to the spec.
";

pub fn entry(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    //let args: Vec<_> = env::args().collect();
    
    if args.len() < 5 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
    let api_key = &args[2];
    let file_path = &args[3];
    let to_sleep_str = &args[4];
	let to_sleep: u64 = to_sleep_str.parse().expect("to sleep argument was not a number i32");

    let f = fs::read_to_string(file_path)?;
    
    let lines: Vec<&str> = f.lines().collect(); 

    let mut matrix: Vec<Vec<&str>> = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i].split('|').collect();
        matrix.push(line);
    }

    //println!("{:?}", matrix[3][0]);

    let mut first_run_line_index: i32 = matrix[3][0].parse()?;
    first_run_line_index -= 1;
    //println!("{:?}", matrix[first_run_line_index as usize]);
	//let variables_raw = fetch("v1/games/nd28p43d/variables", "variables2");
/*
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
*/
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

        if !matrix[i][0].is_empty() && matrix[i][0] != "NO" {
            payload.push_str(&format!(" \"date\": \"{}\",\n", &matrix[i][0]));
        }

        if !matrix[i][1].is_empty() && matrix[i][1] != "NO" {
            payload.push_str(&format!(" \"region\": \"{}\",\n", &matrix[i][1][0..8]));
        }

        if !matrix[i][2].is_empty() && matrix[i][2] != "NO" {
            payload.push_str(&format!(" \"platform\": \"{}\",\n", &matrix[i][2][0..8]));
        }

        if !matrix[i][3].is_empty() && matrix[i][3] != "NO" {
            payload.push_str(" \"emulated\": true,\n",);
        } else {
            payload.push_str(" \"emulated\": false,\n",);
        }
        
        if !matrix[i][5].is_empty() && matrix[5][0].to_lowercase() == "yes" { //IGT
			payload.push_str(" \"verified\": true,\n");
		}
		
		let mut should_next_comma_left2 = false;
        if !matrix[i][4].is_empty() && matrix[i][4] != "NO" {
			payload.push_str(" \"players\": [\n");
			let player_vec = matrix[i][4].split(',');
			for player in player_vec {
				if should_next_comma_left2 {
					payload.push_str(",\n");
				}
				if player.contains("(guest)") {
					payload.push_str(&format!("  {{\"rel\": \"guest\", \"name\": \"{}\"}}", &player[7..]));
				} else {
					payload.push_str(&format!("  {{\"rel\": \"user\", \"id\": \"{}\"}}", player));
				}
				should_next_comma_left2 = true;
			}
			payload.push_str("\n ],\n");
		}//----players will not implement until coop submissions work for moderators again
		
		//times
        payload.push_str(" \"times\": {\n");
        let mut should_next_comma_left = false;
		if !matrix[i][6].is_empty() && matrix[i][6] != "NO" { //LRT
			payload.push_str(&format!("  \"realtime_noloads\": {}", parse_to_seconds(&matrix[i][6]) ) );
			should_next_comma_left = true;
		}
		if !matrix[i][7].is_empty() && matrix[i][7] != "NO" { //RTA
			if should_next_comma_left {
				payload.push_str(",\n");
			}
			payload.push_str(&format!("  \"realtime\": {}", parse_to_seconds(&matrix[i][7]) ) );
			should_next_comma_left = true;
		}
		if !matrix[i][8].is_empty() && matrix[i][8] != "NO" { //IGT
			if should_next_comma_left {
				payload.push_str(",\n");
			}
			payload.push_str(&format!("  \"ingame\": {}", parse_to_seconds(&matrix[i][8]) ) );
			//should_next_comma_left = true;
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
		
		let mut pre_patch_payload = payload.clone();
			//======= //shit
			/*
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
			}*/
			//=======
		payload.push_str("\n }");// close variables
		pre_patch_payload.push_str("\n }");// close variables
		
		
		
		payload.push_str("\n}}");
		pre_patch_payload.push_str("\n}}");
		//==============
        runs.push(pre_patch_payload.clone());
        ppp_runs.push(payload.clone());
     //   println!("{}", payload);
        //println!("{}", matrix[i as usize][5]);
    }
    
    for i in 0..ppp_runs.len() {
		println!("{}", ppp_runs[i]);
	}
//println!("{:?}", vars_per_level);
	println!("==================\nSubmit the speedruns? (yes or no)");
	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text)?;
    let trimmed = input_text.trim().to_lowercase();
    
    if trimmed == "no" {
		/*
		for i in 0..runs.len() {
			println!("{}", runs[i]);
		}
		*/
		return Ok(())
	}
	//==================
	
	
	//==================
	for i in 0..runs.len() {
		let response = ureq::post("https://www.speedrun.com/api/v1/runs")
			.header("Content-Type", "application/json")
			.header("X-Api-Key", api_key)
			.send(&runs[i]);

		println!("{:?}", response);
		println!("Sleeping for 2s");
		//sleep(Duration::new(2, 0));
		sleep(Duration::from_millis(to_sleep));
	}
	println!("=========\nDone. Check your pending.");
	return Ok(())
}
