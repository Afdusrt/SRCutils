use std::{process, fs};

fn time_code_to_seconds(s: &str) -> u32 {
	let hours: u32 = s[0..2].parse().unwrap();
	let minutes: u32 = s[3..5].parse().unwrap();
	let seconds: u32 = s[6..8].parse().unwrap();
	
	return hours*60*60+minutes*60+seconds
}

pub fn process_edl(csv_file_path: &str, video_link: &str) -> bool {
	let contents = match fs::read_to_string(csv_file_path) { //matching the result into the thing
		Ok(contents) => contents,
		Err(error) => { eprintln!("!! Failed to read '{}': {}", csv_file_path, error); return false }
	};
	
	let lines: Vec<&str> = contents.lines().collect();

	let mut csv_load = String::new();
	//filter lines
	let mut filtered_lines: Vec<&str> = Vec::new();
	//let mut filtered_lines: Vec<String> = Vec::new();
	for i in 0..lines.len() {
		if lines[i].starts_with('*') {
			if let Some((_, clip_name)) = lines[i].split_once(':') { //edl formatting bloat removal -- (_, clip_name) _ means it ignores the left side, and clip_name is what the right side gets assigned to, because of what split_once returns
				if let Some((clip_name, _)) = clip_name.rsplit_once('.') { //file extension removal
					//assemble the filtered_lines vec
					//filtered_lines.push( clip_name.trim() ); //file name
					
					let mut h = "0";
					let mut m = "0";
					let mut s = "0";
					let mut ms = "0";
					
					if let Some((level_name, run_time_raw)) = clip_name.split_once("--") {
						let parts: Vec<&str> = run_time_raw.split('-').collect();
						match parts.len() {
							1 => { ms = parts[0] }, //ms
							2 => { s = parts[0]; ms = parts[1] }, //s ms
							3 => { m = parts[0]; s = parts[1]; ms = parts[2] }, //m s ms
							4 => { h = parts[0]; m = parts[1]; s = parts[2]; ms = parts[3] }, // h m s ms
							_ => { eprintln!("line filtering: {}", lines[i]); eprintln!("!! failed at '-' split"); }
						}
						drop(parts);
						
						let parts: Vec<&str> = lines[i-1].split_whitespace().collect(); //time stamp hh:mm:ss:ff
						let yt_timestamp = time_code_to_seconds(parts[6]).to_string();
						
						//csv load
							csv_load.push_str(level_name.trim());
						csv_load.push('°');
							csv_load.push_str(h);
						csv_load.push('°');
							csv_load.push_str(m);
						csv_load.push('°');
							csv_load.push_str(s);
						csv_load.push('°');
							csv_load.push_str(ms);
						csv_load.push('°');
							csv_load.push_str(parts[6]);
						csv_load.push('°');
							csv_load.push_str(video_link);
							csv_load.push_str("&t=");
							csv_load.push_str(&yt_timestamp);
						csv_load.push('\n');
					} else {
						eprintln!("line filtering: {}", lines[i]);
						eprintln!("!! failed at '--' split");
						return false;
					}
				} else {
					eprintln!("line filtering: {}", lines[i]);
					eprintln!("!! failed at '.' split");
					return false;
				}
			} else {
				eprintln!("line filtering: {}", lines[i]);
				eprintln!("!! failed at ':' split");
				return false;
			}
		}
	}
	
	println!("{csv_load}");
	
	match fs::write("output.csv", &csv_load) { //matching the option into the thing
		Ok(_) => { println!("csv file successfully saved as output.csv"); return true }
		Err(error) => { println!("failed to write output.csv: {}", error); return false }
	}
}
