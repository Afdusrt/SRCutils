use std::{ env, fs, io, process, process::Command, collections::HashSet, collections::HashMap };
use json::JsonValue;

const HELP: &str = "HELP:
====
arg 1 - csv sheet
=
This script goes through each player column in the matrix, to get all unique ones, fetches the ids for them and then replaces the usernames with the ids.
Outputs RESOLVED-IDS-csv_sheet
";

fn write_csv(matrix: Vec<Vec<String>>, filename: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename)?;
    for row in matrix {
        writeln!(file, "{}", row.join("|"))?;
    }
    Ok(())
}

fn fetch_raw(s: &str) -> JsonValue {
	let url = format!("https://www.speedrun.com/api/{}", s);
	
	let output = Command::new("curl")
        .arg("-s")
        .arg(url)
        .output()
        .expect("Failed to execute curl");

    let text = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
    let parsed = json::parse(text).expect("Failed to parse JSON");

    parsed
}

fn main() {
	
	let args: Vec<_> = env::args().collect();
    
    if args.len() < 2 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
	let file_path = &args[1];
	
	let f = fs::read_to_string(file_path).unwrap();
    
    let lines: Vec<&str> = f.lines().collect(); 

    let mut matrix: Vec<Vec<String>> = Vec::new();
    
    for line in lines.iter() {
		let mut row: Vec<String> = Vec::new();
		let parts: Vec<&str> = line.split('|').collect();
		
		for i in 0..parts.len() {
			let s = parts[i].to_string();
			row.push(s);
		}
		
        matrix.push(row);
    }
	
    let mut first_run_line_index: usize = matrix[3][0].parse().expect("");
    first_run_line_index -= 1;
    
    let mut unique_players = HashSet::new();
    
    for i in first_run_line_index..matrix.len() {
		println!("{}", matrix[i][4]);
		for player in matrix[i][4].split(',') {
			let trimmed = player.trim();
			
			if trimmed.contains("(guest)") {
				continue
			} else {
				unique_players.insert(trimmed.to_string());
			}
		}
	}
	
	let mut user_ids: HashMap<String, String> = HashMap::new();
	
	for player in &unique_players {
        println!("{}", player);
        let gus = fetch_raw(&format!("v2/GetUserSummary?Url={}", player));
        
        println!("{}", gus["user"]["id"]);
        let id = gus["user"]["id"].as_str().unwrap().to_string();
        user_ids.insert(player.clone(), id);
    }
    
    for i in first_run_line_index..matrix.len() {
        let replaced: Vec<String> = matrix[i][4]
            .split(',')
            .map(|player| {
                let player = player.trim();
                
				if player.contains("(guest)") {
					return player.to_string();
				}
				
                match user_ids.get(player) {
                    Some(id) => id.clone(),
                    None => player.to_string(),
                }
            })
            .collect();

        matrix[i][4] = replaced.join(",");
    }
    
    let file_to_save = format!("RESOLVED-IDS-{}", file_path);
	println!("===============\nSpreadsheet saved as: {}", file_to_save);
	write_csv(matrix, &file_to_save);
}
