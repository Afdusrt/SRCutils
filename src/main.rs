use std::env;
use std::fs;
use std::process::Command;

const USAGE_ERROR_MESSAGE: &str = "
=============
 Read:
  Dependecies for network functionality: curl on path.
  
  Run this in a directory where you have read and write permission
  
  The edl parsing functionality of this program will not work if:
	the IL has -- or ° in the name

 Usage:
  arguments (* = optional, ** = almost optional):
	to use default value, input //
	first argument decides the mode, 
		MODE: edl - edl (CMX) parser -> csv file (in reality its °sv)
		      -- youtube_video_link edl_file_path
			  
		      sub - submitter of runs from csv file
		      -- game_abbreviation dsv_file_path example_command.txt
		      
		      - for example_command.txt, submit one run with the network tools open, and then copy that request as Curl(posix)
		      
		      --IMPORTANT: when you submit a run, fill in all fields: hours, minutes, seconds, milliseconds, description, video link
=============
";

mod edlparser;
mod runsubmitter;

fn main() {
	let args: Vec<_> = env::args().collect();
	
	//let modifier = &args[args.len() - 1]; //last arg
	
	if args.len() < 2 {
		panic!("{}", USAGE_ERROR_MESSAGE);
	}
	
	if args[1] == "edl" {
		println!("mode is edl - edl parser");
		if args.len() < 3 {
			panic!("{}", USAGE_ERROR_MESSAGE);
		}
		
		let yt_video_link = &args[2];
		
		let mut file_path = "edl.edl";
		
		if &args[3] != "//" {
			file_path = &args[3];
		}
		
		edlparser::process_edl(file_path, yt_video_link);
	}
	
	if args[1] == "sub" {
		println!("mode is sub - run submitter");
		
		if args.len() < 3 {
			panic!("{}", USAGE_ERROR_MESSAGE);
		}
		
		//let mut example_command_path = "example_command.txt";
		//let mut dsv_file_path = "output.csv";
		
		let game_abbreviation = &args[2];
		let dsv_file_path = &args[3];
		let example_command_path = &args[4];
		let modifier = &args[args.len() - 1];
		
		println!("game {game_abbreviation}");
		println!("dsv {dsv_file_path}");
		println!("command {example_command_path}");
		println!("==========");
		
		runsubmitter::submit_runs(game_abbreviation, dsv_file_path, example_command_path, modifier);
	}
}
