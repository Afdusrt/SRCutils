use std::process::Command;
use std::fs;
use std::time::SystemTime;

fn decode_weird_unicode(s: &str) -> String {
	let mut result = String::new();
	let mut chars = s.chars().peekable();
	
	while let Some(c) = chars.next() {
		if c == '\\' && chars.peek() == Some(&'u') { //check if there is a sorta \uFFFF 
			chars.next();
			let hex: String = chars.by_ref().take(4).collect();
			if let Ok(code_point) = u32::from_str_radix(&hex, 16) { //convert hex unicode string to u32
			if let Some(ch) = std::char::from_u32(code_point) {
					result.push(ch);
				}
			}
		} else {
			result.push(c);
		}
	}
	
	result
}

pub fn submit_runs(game_abbreviation: &str, dsv_file_path: &str, example_command_path: &str, modifier: &str) {
	//get levels list json
	if game_abbreviation != "//" {
		Command::new("curl")
			.arg("-L")
			.arg(format!("http://www.speedrun.com/api/v1/games/{}/levels", game_abbreviation))
			.arg("-o")
			.arg("levels.json")
			.status()
			
			.expect("Failure");
	}
	
	let contents = fs::read_to_string(dsv_file_path)
		.expect("Didnt read file");
		
	let raw_levels_json = fs::read_to_string("levels.json")
		.expect("Didnt read file");
	
	let levels_json = decode_weird_unicode(&raw_levels_json);
	
	let example_command = fs::read_to_string(example_command_path)
		.expect("Didnt read file");
	
	//example command parsing
	let example_lines: Vec<&str> = example_command.lines().collect();
	
	let mut cookie_argument = "";
	for i in 0..example_lines.len() - 1 {
		if example_lines[i].contains("Cookie") {
			cookie_argument = &example_lines[i][6..(example_lines[i].len() - 3)];
			//println!("{}", cookie_argument);
			//println!("==========");
		}
	}
	
	//get time since UNIX_EPOCH date
	let new_date_text = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
	
	let mut data_argument = String::from("");
	//replace example levelId with placeholder ========
	for i in 0..example_lines.len() - 0 {
		if example_lines[i].contains("--data-raw") {
			
			let data_argument_line = &example_lines[i][14..(example_lines[i].len() - 1)];
			
			//replace existing levelId with placeholder
			let levelId_index = data_argument_line.find("levelId").unwrap();
			
			let example_levelId = &data_argument_line[levelId_index + 10..levelId_index + 18];
			
			let data_argument_with_levelId_placeholder = data_argument_line.replace(example_levelId, "========");
			//=========================================

			data_argument = data_argument_with_levelId_placeholder;
		}
	}
	
	//DSV rows actions...
	let mut log = String::new(); //string for logging failed requests
	
	let lines: Vec<&str> = contents.lines().collect();
	//iterate over each row in the dsv file and submit runs according to that field
	//for i in 0..2 {
	for i in 0..lines.len() {
		let mut parts: Vec<&str> = lines[i].split("°").collect();
		// 0 level name | 1 hours | 2 minutes | 3 seconds | 4 milliseconds | 5 description | 6 video link
		
		if parts[1] == "" {
			parts[1] = "0";
		}
		if parts[2] == "" {
			parts[2] = "0";
		}
		if parts[3] == "" {
			parts[3] = "0";
		}
		if parts[4] == "" {
			parts[4] = "0";
		}
		
		let parts0formatted = format!("{}\"", parts[0]);
		//let level_name_index = levels_json.find(parts[0]).unwrap(); //find where the level name is in the json file
		
		//let level_name_index = levels_json.find(&parts0formatted).unwrap(); //find where the level name is in the json file
		
		let mut level_id = String::new();
		
		if let Some(level_name_index) = levels_json.find(&parts0formatted) {
			level_id = (&levels_json[(level_name_index - 18)..(level_name_index - 10)]).to_string(); //go back a couple characters to get the level id next to the name
		} else {
			println!("Cannot find level: {}, skipping this i", &parts0formatted);
			let log_msg = format!("Line {}: {}\n", i, parts0formatted);
			log.push_str(&log_msg);
			continue;
		}
		
		//let level_id = &levels_json[(level_name_index - 18)..(level_name_index - 10)]; //go back a couple characters to get the level id next to the name
		
		//println!("\n==========");
		//println!("{} -- {}", parts[0], &level_id);
		//println!("==========");
		
		//data_argument = data_argument.replace("========", level_id);
		
		//println!("{}", data_argument);
		
		//find igt field in data_argument and insert according values 
		if let Some(igt_index) = data_argument.find("igt") {
			let almost_igt_block = &data_argument[igt_index..];
			let igt_block_ending_index = almost_igt_block.find(r"}").unwrap();
			let igt_block = &almost_igt_block[..igt_block_ending_index+1];
			
			//println!("{:?}", &igt_block);
			//println!("==========");
			
			let new_igt_block = format!(
				"igt\":{{\"hour\":{},\"minute\":{},\"second\":{},\"millisecond\":{}}}", parts[1], parts[2], parts[3], parts[4]
			);
			
			data_argument = data_argument.replace(igt_block, 
				&new_igt_block
			);
			
			//println!("{}", data_argument);
			//println!("==========");
			
			//get ending of igt block
		} //if igt isnt there dont put values in it
		
		//find rta field in data_argument and insert according values
		if let Some(rta_index) = data_argument.find("time") {
			let almost_rta_block = &data_argument[rta_index..];
			let rta_block_ending_index = almost_rta_block.find(r"}").unwrap();
			let rta_block = &almost_rta_block[..rta_block_ending_index + 1];
			
			//println!("{:?}", &rta_block);
			//println!("==========");
			
			let new_rta_block = format!(
				"time\":{{\"hour\":{},\"minute\":{},\"second\":{},\"millisecond\":{}}}", parts[1], parts[2], parts[3], parts[4]
			);
			
			data_argument = data_argument.replace(rta_block, 
				&new_rta_block
			);
			//println!("{}", data_argument);
			//println!("==========");
			
			//get ending of rta block
		} //if rta isnt there dont put values in it
		
		//replace video link in data_argument with according one
		if let Some(video_index) = data_argument.find("video") {
			let almost_video_block = &data_argument[video_index..];
			let video_block_ending_index = almost_video_block.find(r"}").unwrap();
			let video_block = &almost_video_block[..video_block_ending_index + 1];
			
			//println!("{:?}", &video_block);
			let videolink_ending_index = video_block.find(",").unwrap();
			
			let videolink = &video_block[8..videolink_ending_index - 1];
			
			let new_video_block = video_block.replace(videolink, parts[6]);
			
			data_argument = data_argument.replace(video_block, 
				&new_video_block
			);
			//println!("{}", data_argument);
			//println!("==========");
			
			//println!("{:?}", &videolink);
			//println!("{:?}", new_video_block);
		}
		
		//replace commect in data_argument with according one
		if let Some(comment_index) = data_argument.find("comment") {
			let almost_comment_block = &data_argument[comment_index..];
			let comment_block_ending_index = almost_comment_block.find(r"}").unwrap();
			let comment_block = &almost_comment_block[..comment_block_ending_index + 1];
			
			let commenttext_ending_index = comment_block.find("}").unwrap();
			
			let commenttext = &comment_block[10..commenttext_ending_index - 1];
			
			let new_comment_block = comment_block.replace(commenttext, parts[5]);
			
			//println!("{:?}", &new_comment_block);
			
			data_argument = data_argument.replace(comment_block, 
				&new_comment_block
			);
		}
		
		//replace date in example_command with current date
		if let Some(date_index) = data_argument.find("date") {
			let almost_date_block = &data_argument[date_index..];
			let date_block_ending_index = almost_date_block.find(r",").unwrap();
			let date_block = &almost_date_block[..date_block_ending_index + 1];
			
			let date_text_ending_index = date_block.find(",").unwrap();
			
			let date_text = &date_block[6..date_text_ending_index];
			

			let new_date_block = &date_block.replace(date_text,
				&new_date_text.to_string()
			);

			data_argument = data_argument.replace(date_block, 
				&new_date_block
			);
		}
		
		//put level id
		let new_data_argument = data_argument.replace("========", &level_id);
		println!();
		println!("\n{} -- {}", parts[0], &level_id);
		println!("{}", new_data_argument); //print final payload
		
		if modifier != "//" {
			Command::new("curl")
				.arg("https://www.speedrun.com/api/v2/PutRunSettings")
				.arg("-X")
				.arg("POST")
			
				.arg("-H")
				.arg("Accept: application/json")
				.arg("-H")
				.arg("Content-Type: application/json")
				.arg("-H")
				.arg(cookie_argument)
				.arg("--data-raw")
				.arg(new_data_argument)
			
				.status().expect("Failure");
		}
	}
	println!("\n==== Missing Level Log ====\n{}\n", log);
	fs::write("Log.txt", &log).expect("Could not write log file");
	println!("Missing levels log saved to \"Log.txt\"")
}
