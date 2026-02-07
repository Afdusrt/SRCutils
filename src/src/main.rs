const ARGS_ERROR_MESSAGE: &str = "====
ERROR: program ran with insufficient arguments
====
";
const HELP: &str = "HELP:
====
arg 1 - Mode selection: edl OR sub
=
edl mode:
arg 2 - youtube link as prefix to &t=x
arg 3 - compatible edl (details in docs) file path

sub mode: (does not work right now)
arg 2 - game abbreviation
arg 3 - compatible csv (details in docs) file path
arg 4 - example command (details in docs) file path
";
mod edlparser;
use std::{env, process, process::Command};

fn check_dependancy_curl() -> bool {
	Command::new("curl")
		.arg("--version")
		.output()
	.is_ok()
}

fn main() {
	let args: Vec<_> = env::args().collect();

	if args.len() == 1 { //no args mean .len() is 1
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
	match args[1].as_str() {
		"edl" => {
			println!("====");
			println!("mode is edl - edl parser\n");
		
			if args.len() != 4 {
				eprintln!("{}", ARGS_ERROR_MESSAGE);
				eprintln!("{}", HELP);
				process::exit(1);
			}
		
			let yt_video_link = &args[2];
			let csv_file_path = &args[3];
		
			if !edlparser::process_edl(csv_file_path, yt_video_link) {
				eprintln!("====");
				eprintln!("!! edl processing failed, viz output");
				process::exit(3);
			};
		},
		"sub" => {
			//NOT DONE BECAUSE OF CLOUDFLARE
			if check_dependancy_curl() != true {
				eprintln!("curl not available");
				process::exit(2);
			}
		},
		_ => {
			eprintln!("{}", ARGS_ERROR_MESSAGE);
			eprintln!("{}", HELP);
			process::exit(1);
		}
	}
}
