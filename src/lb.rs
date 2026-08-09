use std::fs;
use std::cmp::Reverse;
//use std::path::PathBuf;
use crate::things::LeaderboardResponse;
use crate::things::User;
use crate::things::compute_user;

pub fn entry(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	//defaults
	let mut in_folder = "fetchGUL";
	let mut out_folder = "leaderboards";
	
	match args.len() {
		2 => {},
		3 => { in_folder = &args[2] },
		4 => { in_folder = &args[2]; out_folder = &args[3] },
		_ => { return Err("Too many arguments, typo?".into()); }
	}
	
	let files = fs::read_dir(in_folder)?;
	//println!("{:?}", files.collect::<Vec<String>>());
	let mut users: Vec<User> = Vec::new();
	
	for (i, entry) in files.enumerate() {
		if (i >= 100) && (i % 100 == 0) {
			println!("processed 100");
		}
		let entry = match entry {
			Ok(e) => e,
			Err(_) => continue,
		};

		let path = entry.path();
		//println!("{:?}", path);
		
		let path_data = fs::read_to_string(&path)?;
		
		let data: LeaderboardResponse = serde_json::from_str(&path_data)?;
		
		let mut user = compute_user(data);
		if user.areaId.len() > 2 {
			user.areaId = user.areaId[0..2].to_string();
		}
		users.push(user);
	}

	fs::create_dir_all(out_folder)?;
	//all users have the same leaderboards, so gather leaderboards from index 0
	let leaderboards = &users[0].lbs;
	for (i, leaderboard) in leaderboards.iter().enumerate() {
		let mut sorted: Vec<&User> = users.iter().collect(); //pre-sort
		
		sorted.sort_by_key( |user| Reverse(user.lbs[i].value.score()) );
		
		let mut out = String::new();
		
		for (place, user) in sorted.iter().take(leaderboard.lb_size).enumerate() {
            out.push_str(&format!(
				"`{:<4}`{}`{:<20}{}`\n",
				format!("{}.", place + 1),
				if user.areaId.is_empty() { ":united_nations:".to_string() } else { format!(":flag_{}:", user.areaId) },
				user.name,
				user.lbs[i].value.score()
			));
        }
        
        fs::write(format!("{}/{}.txt", out_folder, leaderboard.name), out)?;
	}
	return Ok(())
}
