use std::fs;

use std::io::ErrorKind;
use std::collections::HashSet;

use crate::things;

//pub fn entry(args: &Vec<String>) -> Result<(), &'static str> {
pub fn entry(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	//defaults
	let mut database_path = "online";
	let mut out_folder = "fetchGUL";
	
	match args.len() {
		2 => {},
		3 => { database_path = &args[2] },
		4 => { database_path = &args[2]; out_folder = &args[3] },
		_ => { return Err("Too many arguments, typo?".into()); }
	}
	let database = match database_path {
			"online" => { println!("Fetching online database 'https://gist.githubusercontent.com/Afdusrt/e78b52fdcb366517646abeee1362692e/raw/'");
						  things::get("https://gist.githubusercontent.com/Afdusrt/e78b52fdcb366517646abeee1362692e/raw/")?
						},
			_ => { fs::read_to_string(database_path)? }
		};
		
	let starting_over: bool = match fs::create_dir(out_folder) {
			Ok(()) => { false },
			Err(e) => match e.kind() {
				ErrorKind::AlreadyExists => {
						if things::ask_bool("output folder exists, do you wanna start over with fetching(y) or continue from before(n)?") {
							fs::remove_dir_all(out_folder)?;
							fs::create_dir(out_folder)?;
							true
						} else {
							false
						}
					},
				_ => return Err(e.into()),
			},
		};
	
	let mut existing_user_ids = HashSet::new();
	
	let existing = fs::read_dir(out_folder)?;
	
	for entry in existing {
		let entry = match entry {
			Ok(e) => e,
			Err(_) => continue,
		};

		if let Some(stem) = entry.path().file_stem() {
			if let Some(id) = stem.to_str() {
				existing_user_ids.insert(id.to_string());
			}
		}
	}
	
	//let user_ids: Vec<&str> = database.lines().map(|item| item.trim()).collect();
	
	//for user_id in user_ids {
	//for user_id in database.lines().map(|item| item.trim()) {
	let mut user_ids: Vec<String> = database
        .lines()
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect();

    for user_id in user_ids.clone() {
		if !starting_over && existing_user_ids.contains(&user_id) {
			println!("Skipping {}", user_id);
			continue;
		}
		
		let user_leaderboard = match things::get_user_leaderboard(&user_id) {
				Ok(res) => res,
				Err(e) => {
					if e.to_string() == "http status: 404" {
                    println!("{} returned 404, removing from database", user_id);

						// Remove the user from our in-memory list.
						user_ids.retain(|id| id != &user_id);

						// Rewrite the database without this user.
						fs::write(database_path, user_ids.join("\n") + "\n")?;

						continue;
					}

					// Any other error is still a real error.
					return Err(e.into());
					
				}
			};
			
		fs::write(format!("{}/{}.json", out_folder, user_id), user_leaderboard)?;
	}
	
	return Ok(())
}
