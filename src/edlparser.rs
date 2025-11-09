fn time_code_to_seconds(s: &String) -> u16 {
	let hours: u16 = s[0..2].parse().unwrap();
	let minutes: u16 = s[3..5].parse().unwrap();
	let seconds: u16 = s[6..8].parse().unwrap();
	
	return hours*60*60+minutes*60+seconds
}

use std::fs;

pub fn process_edl(file_path: &str, video_link: &str) {
	
    let contents = fs::read_to_string(file_path)
		.expect("Didnt read file");

	let lines: Vec<&str> = contents.lines().collect();
	
	let mut csv_as_string = String::from("");
	
	for i in 0..lines.len() {
		if lines[i].contains("AA/V") {
            if i + 1 < lines.len() {
				//println!("====");
				let file_name = &lines[i+1][18..];
				let r = file_name.rfind("."); //find last "." so the extension can get removed

				let name_with_igt = &file_name[..&file_name.len()-(&file_name.len()-r.unwrap())];
				let name_and_igt: &Vec<&str> = &name_with_igt.split("--").collect();
				
				//println!("{:?}", name_with_igt);
				//println!("{:?}", name_and_igt);
				
				let timestamp = String::from(&lines[i][53..65]);
				let video_link_timed = format!("{video_link}&t={}",time_code_to_seconds(&timestamp));
				
				let parts: Vec<&str> = name_and_igt[1].split('-').collect();
				
				//println!("{:?}", parts.len());
				//println!("{:?}", parts);
				
				let level = &name_and_igt[0];
				
				//println!("{:?}", parts.len());
				//println!("{:?}", level);
				
				let mut h = "0";
				if parts.len() > 3 {
					h = &parts[parts.len()-4];
				}
				
				let mut min = "0";
				if parts.len() > 2 {
					min = &parts[parts.len()-3];
				}
				
				let mut s = "0";
				if parts.len() > 1 {
					s = &parts[parts.len()-2];
				}
				
				let ms = &parts[parts.len()-1];
				
				//println!("{level},{h},{min},{s},{ms},{},{}\n", &lines[i][53..64],video_link_timed);
				
				csv_as_string.push_str(&format!("{level}°{h}°{min}°{s}°{ms}°{}°{}\n", &lines[i][53..64],video_link_timed));
			}
        }
	}
	fs::write("output.csv", &csv_as_string).expect("Couldn't write output.csv");
	//println!("{}", csv_as_string);
	println!("=============\ncsv file saved as output.csv, you can run mode sub on it");
}
