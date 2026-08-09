use std::{ fs, process, collections::HashSet, collections::HashMap };

const HELP: &str = "HELP:
====
arg 2 - csv sheet
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

fn fetch(s: &str) -> Result<String, ureq::Error> {
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
}

pub fn entry(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	
	//let args: Vec<_> = env::args().collect();
    
    if args.len() < 3 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
	let file_path = &args[2];
	
	let f = fs::read_to_string(file_path)?;
    
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
	
    let mut first_run_line_index: usize = matrix[3][0].parse()?;
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
        let gus1 = fetch(&format!("v2/GetUserSummary?Url={}", player))?;
        let gus = json::parse(&gus1)?;
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
	write_csv(matrix, &file_to_save)?;
	
	Ok(())
}
