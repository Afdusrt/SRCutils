#![allow(non_snake_case)]

use std::env;

const HELP: &str = "
HELP:
====
Argument 1: mode
   -check-updates: check for updates
   
   -fetch: fetch v2/GetUserLeaderboard data from database, into a folder.
      -arg 2: database path (database format: 1 line = 1 user id)
         -optional, default 'users.txt'
      -arg 3: output path (folder to put json output files into)
         -optional, default 'fetchGUL'

   -lb: create leaderboards txt from a folder from fetch mode, for discord bot.
      -arg 2: folder path
         -optional, default 'fetchGUL'
      -arg 3: output path (folder to put .txt leaderboards into)
         -optional, default 'leaderboards'

   -prepare-sheet: Prepares a spreadsheet, for submit sheet mode
      -arg 2: game abbreviation
      -arg 3: csv file to save
      
   -submit-sheet: Submits speedruns with data from a spreadsheet from prepare-sheet mode.
      -arg 2: api key
      -arg 3: csv file to submit
      -arg 4: sleep time between requests in ms
      
   -timestamp-sheet: timestamp runs from retime notes, from prepare-sheet.
      -arg 2: csv sheet
      -arg 3: base youtube link (that '?t=' can be added after)
         -(you can also input 'inline' to grab a video link from above the VIDEO in the video column
         -(you can also input 'replace' to grab a video link from video column itself)
      -arg 4: time field (RTALRTIGT <- string like this, eg. you can do \"LRTIGT\" for both, \"IGT\" for only igt...)
      
   -resolveids: get userids from a spreadsheet that contains usernames (necessary befory submit-sheet
      -arg 2: csv sheet
      
   -uncredit: uncredit a user's run with a filter
      -arg 2: user name of victim
      -arg 3: guest name that the victim will be forced into
      -arg 4: game abbreviation or id
      -arg 5: api key
      -arg 6: sleep in milliseconds between requests
====
";


mod things;

mod fetch;
mod lb;
mod preparesheet;
mod submitsheet;
mod timestampsheet;
mod resolveids;
mod uncredit;

//fn decypher_mode(args: &Vec<String>) -> Result<(), &'static str> {
fn decypher_mode(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	if args.len() < 2 {
        return Err("Missing mode".into());
    }
    
    match args[1].as_str() {
		"check-updates" => {
			println!("Checking for updates");
			return things::check_update();
		},
		"fetch" => {
			println!("Mode: fetch");
			return fetch::entry(args);
		},
		"lb" => {
			println!("Mode: lb");
			return lb::entry(args);
		},
		"prepare-sheet" => {
			println!("Mode: prepare-sheet");
			return preparesheet::entry(args);
		},
		"submit-sheet" => {
			println!("Mode: submit-sheet");
			return submitsheet::entry(args);
		},
		"timestamp-sheet" => {
			println!("Mode: timestamp-sheet");
			return timestampsheet::entry(args);
		},
		"resolveids" => {
			println!("Mode: resolveids");
			return resolveids::entry(args);
		},
		"uncredit" => {
			println!("Mode: uncredit");
			return uncredit::entry(args);
		},
		_ => { return Err("Invalid mode".into()) }
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	match decypher_mode(&args) {
		Ok(()) => {},
		Err(e) => { eprintln!("ERROR: {e}\n{}", HELP); return }
	}
}
