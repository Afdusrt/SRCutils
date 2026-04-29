use std::{ env, fs, io, process, process::Command, time::{Duration, SystemTime}, thread::sleep, };

fn parse_to_seconds(input: &str) -> i32 {
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
    let sint = s as i32;
    //sint.to_string()
    //s.to_string()
    sint
}

const HELP: &str = "HELP:
====
arg 1 - csv sheet
arg 2 - base youtube link (that '?t=' can be added after)
        -(you can also input 'inline' to grab a video link from above the VIDEO in the video column
        -(you can also input 'replace' to grab a video link from video column itself)
arg 3 - time field (RTALRTIGT <- string like this, eg. you can do \"LRTIGT\" for both, \"IGT\" for only igt...)
=
This is for sheets made with prepare-sheet, it extracts retiming notes from the comment column, and then generates timestamped youtube links.

Note formatting:
example note: Note: Start Time 59:01.133, End Time: 59:44.933, Frame Rate: 30, Time: 43.8

The script seaches for three time values, of format \"1:23.456\", the first one, gets turned into seconds with second subtracted from it, others are ignored except for last one, which gets pasted into the time field/s you select.
";

fn extract_times(note: &str) -> Vec<String> {
	let mut times = Vec::new();
	
	for part in note.split_whitespace() {
		let cleaned: String = part.chars()
            .filter(|c| c.is_digit(10) || *c == ':' || *c == '.')
            .collect();

        if cleaned.is_empty() || !cleaned.chars().any(|c| c.is_digit(10)) {
            continue;
        }
        
        times.push(cleaned)
        /*
        let result = std::panic::catch_unwind(|| parse_to_seconds(&cleaned));
        if let Ok(seconds) = result {
            times.push(seconds);
        }*/
        
		//println!("{:?}", cleaned);
		//println!("{:?}", times);
	}
        
	times
}

fn write_csv(matrix: Vec<Vec<String>>, filename: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename)?;
    for row in matrix {
        writeln!(file, "{}", row.join("|"))?;
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() < 4 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
	let file_path = &args[1];
	let mut yt_link = &args[2];
	let time_field = &args[3];
	
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
    let mut replace_thingamabob = false;
    if yt_link == "replace" {
		replace_thingamabob = true
	}
    let mut yt_link = if yt_link.trim() == "inline" {
		matrix[first_run_line_index - 2][9].clone() // owned String
	} else {
		yt_link.to_string() // convert &str to owned String
	};
    
    for i in first_run_line_index..matrix.len() {
		if replace_thingamabob {
			yt_link = matrix[i][9].clone();
		}
		//==============
		let note = &matrix[i][10];
		
		let times = extract_times(&note);
		println!("Extracted seconds: {:?}", times);
		
		if times.is_empty() { continue };
		
		let timestamp = parse_to_seconds(&times[0]) - 1;
		let timestamped_link = format!("{}?t={}", yt_link, timestamp);
		println!("{}", timestamped_link);
		
		matrix[i][9] = timestamped_link;
		
		let final_time = &times[times.len() -1];
		//==============
		if time_field.to_lowercase().contains("lrt") {
			matrix[i][6] = final_time.to_string()
		}
		if time_field.to_lowercase().contains("rta") {
			matrix[i][7] = final_time.to_string()
		}
		if time_field.to_lowercase().contains("igt") {
			matrix[i][8] = final_time.to_string()
		}
	}
	
	let file_to_save = format!("TIMESTAMPTED-{}", file_path);
	println!("===============\nSpreadsheet saved as: {}", file_to_save);
	write_csv(matrix, &file_to_save);
}
