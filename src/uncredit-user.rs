use std::{ env, fs, io, process, process::Command, time::{Duration, SystemTime}, thread::sleep, collections::HashMap };
use json::JsonValue;

#[derive(Debug)]
struct Variable {
	id: String,
	name: String,
	if_il_id: String,
	options: Vec<String>,
}

#[derive(Debug)]
struct Category {
	id: String,
	name: String,
	is_il: bool,
	variables: Vec<Variable>,
	variables_per_level: Vec<Variable>
}

#[derive(Debug)]
struct Variable_to_filter {
	id_name: String,
	selected_option_id_name: String
}
/*
fn fetch_raw(s: &str, n: &str) -> String {
	Command::new("curl")
		.arg("-L")
		.arg(format!("http://www.speedrun.com/api/{}", s))
		.arg("-o")
		.arg(n)
		.status()
	.expect("Failure");
	
	let raw_json = fs::read_to_string(n)
		.expect("Didnt read file");
	
	//let json = decode_weird_unicode(&raw_json);
	
	fs::write(n, &raw_json).expect("Could not write file");
	
	return raw_json
}
*/
fn fetch_raw(s: &str) -> JsonValue {
	let url = format!("https://www.speedrun.com/api/{}", s);
	
	let output = Command::new("curl")
        .arg("-s")
        .arg(url)
        .output()
        .expect("Failed to execute curl");

    let text = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
    let parsed = json::parse(text).expect("Failed to parse JSON");

    parsed
}

fn fetch_runs(user_id: &str, game_id: &str, offset: usize, max: usize) -> JsonValue {
    let url = format!("https://www.speedrun.com/api/v1/runs?user={}&game={}&offset={}&max={}", user_id, game_id, offset, max);

    let output = Command::new("curl")
        .arg("-s")
        .arg(url)
        .output()
        .expect("Failed to execute curl");

    let text = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
    let parsed = json::parse(text).expect("Failed to parse JSON");

    parsed
}

fn fetch_users(user_name: &str) -> JsonValue {
    let url = format!("https://www.speedrun.com/api/v1/users/{}", user_name);

    let output = Command::new("curl")
        .arg("-s")
        .arg(url)
        .output()
        .expect("Failed to execute curl");

    let text = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
    let parsed = json::parse(text).expect("Failed to parse JSON");

    parsed
}

fn fetch_all_runs(user_id: &str, game_id: &str) -> Vec<JsonValue> {
    let mut all_runs = Vec::new();
    let mut offset = 0;
    let max = 200;

    loop {
        let page = fetch_runs(user_id, game_id, offset, max);
        let data = &page["data"];

        if data.len() == 0 {
            break;
        }

        for run in data.members() {
            all_runs.push(run.clone());
        }

        if data.len() < max {
            break;
        }

        offset += max;
        
        println!("Progress (how many runs have been gathered): {}", offset);
    }

    all_runs
}

const HELP: &str = "HELP:
====
arg 1 - user name of victim
arg 2 - guest name that the victim will be forced into
arg 3 - game id
arg 4 - api key
arg 5 - sleep in milliseconds between requests
=
You will be prompted things. You need to have curl installed on your path, this program relies on Command::new(\"curl\")
";

fn main() {
	let args: Vec<_> = env::args().collect();
	
	if args.len() < 4 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
	let user_name = &args[1];
	let guest_name = &args[2];
	let gameId = &args[3];
	let api_key = &args[4];
	let to_sleep_str = &args[5];
	let to_sleep: u64 = to_sleep_str.parse().expect("to sleep argument was not a number i32");
	/*
	let user_summary = fetch_raw(&format!("v2/GetUserSummary?Url={}", user_name), "user_summary");
    let parsed_user_summary = json::parse(&user_summary).unwrap();
    */
    let parsed_user_summary = fetch_raw(&format!("v2/GetUserSummary?Url={}", user_name));
	let userId = parsed_user_summary["user"]["id"].as_str().unwrap();
	/*
	let userId = "8w12903x"; //VANCANTORUS
	let guest_name = "[]vancantorus"; //VANCANTORUS
	let gameId = "3dxynpy6"; //VS Homer
	let api_key = "";
	*/
	/*
	let ggd_raw = fetch_raw(&format!("v2/GetGameData?gameId={}", gameId), "ggd");
    let parsed_ggd = json::parse(&ggd_raw).unwrap();
    */
    let parsed_ggd = fetch_raw(&format!("v2/GetGameData?gameId={}", gameId));
    
    let mut archived_map_vars: HashMap<String, bool> = HashMap::new();
    let mut archived_map_options: HashMap<String, bool> = HashMap::new();

for i in 0..parsed_ggd["variables"].len() {
    let id = parsed_ggd["variables"][i]["id"].as_str().unwrap().to_string();
    let archived = parsed_ggd["variables"][i]["archived"]
        .as_bool()
        .unwrap_or(false);

    archived_map_vars.insert(id, archived);
}
for i in 0..parsed_ggd["values"].len() {
    let id = parsed_ggd["values"][i]["id"].as_str().unwrap().to_string();
    let archived = parsed_ggd["values"][i]["archived"]
        .as_bool()
        .unwrap_or(false);

    archived_map_options.insert(id, archived);
}
	/*
	let categories_raw = fetch_raw(&format!("v1/games/{}/categories", gameId), "categories");
	let parsed_categories = json::parse(&categories_raw).unwrap();
	*/
	let parsed_categories = fetch_raw(&format!("v1/games/{}/categories", gameId));
	let mut cats: Vec<Category> = Vec::new();
	
	println!("Pick category for runs to be uncredited:\n========");
		for i in 0..parsed_categories["data"].len() {
			let mut is_il = false;
			if parsed_categories["data"][i]["type"].to_string() == "per-level" {
				is_il = true
			}
			cats.push( Category {
				id: parsed_categories["data"][i]["id"].to_string(),
				name: parsed_categories["data"][i]["name"].to_string(),
				is_il: is_il,
				variables: Vec::new(),
				variables_per_level: Vec::new()
			});
		}
		
		for i in 0..cats.len() {
			println!("{}/ {}/ {}/ {}", i, cats[i].id, cats[i].name, cats[i].is_il);
		}
	
	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let selection: i32 = input_text.trim().parse().expect("string was not intable");
    //if modifier == "debug" {
		println!("You picked category: {}", cats[selection as usize].name);
	//}
    let mut selected_cat = &mut cats[selection as usize];
    
    //
    /*
    let variables_raw = fetch_raw(&format!("v1/categories/{}/variables", selected_cat.id), "variables");
    //let variables2_raw = fetch(&format!("v1/games/{}/variables", game_abbreviation), "variables2");
    let parsed_variables = json::parse(&variables_raw).unwrap();
    */
    let parsed_variables = fetch_raw(&format!("v1/categories/{}/variables", selected_cat.id));
    let mut vars: Vec<Variable> = Vec::new();
    let mut vars_per_level: Vec<Variable> = Vec::new();
    
    	for i in 0..parsed_variables["data"].len() {
		let id = parsed_variables["data"][i]["id"].as_str().unwrap().to_string();

		//cross reference ggd
		let is_archived = archived_map_vars.get(&id).copied().unwrap_or(false);

		if is_archived {
			continue;
		}
		
		//if modifier == "debug" {
			//println!("{}/ {}", id, parsed_variables["data"][i]["name"]);
		//}
		
		let mut is_per_level = false;
		let mut if_il_id = String::new();

		if parsed_variables["data"][i]["scope"]["type"] == "single-level" {
			if_il_id = parsed_variables["data"][i]["scope"]["level"].to_string();
			is_per_level = true;
		}

		let mut options: Vec<String> = Vec::new();
		for (opt_id, name) in parsed_variables["data"][i]["values"]["values"].entries() {
			let id = opt_id.clone().to_string();
			
			let is_archived = archived_map_options.get(&id).copied().unwrap_or(false);
			
			//let is_archived = archived_map_options.get(&id).copied().unwrap_or(false);
			
			if is_archived {
				continue;
			}
			
			options.push(format!("{}/ {}", opt_id, name["label"]));
		}

		let var = Variable {
			id: id.clone(),
			name: parsed_variables["data"][i]["name"].to_string(),
			if_il_id,
			options,
		};

		if is_per_level {
			vars_per_level.push(var);
		} else {
			vars.push(var);
		}
	}
	//cats[selection as usize].variables = vars;
	selected_cat.variables = vars;
	selected_cat.variables_per_level = vars_per_level;
    //
    println!("Variable selections:\n========");
    
    println!("Which variables do you want to filter for? aka, runs with these variables and these options, will be uncredited: (answer format: \"1,3,5\")\n========");
    for i in 0..selected_cat.variables.len() {
		println!("{} {}", i, selected_cat.variables[i].name);
	}
	
	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let selections: Vec<_> = input_text.trim().split(',').collect();
    let mut selected_vars: Vec<i32> = Vec::new();
    for selection in &selections {
		let thing: i32 = selection.parse().expect("variable selection didnt not return ints");
		selected_vars.push(thing)
	}
    //println!("{:?}", selected_vars);
    
    println!("========");
    let mut variables_to_filter: Vec<Variable_to_filter> = Vec::new();
    println!("Which options?:");
    for index1 in 0..selected_vars.len() {
		println!("{}", selected_cat.variables[selected_vars[index1] as usize].name); //for this variable
		for i in 0..selected_cat.variables[selected_vars[index1] as usize].options.len() { //these options
			println!("-{} {}", i, selected_cat.variables[selected_vars[index1] as usize].options[i]);
		}
		let mut input_text = String::new();
		io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
		let selection: i32 = input_text.trim().parse().expect("string was not intable");
		let selected_option = &selected_cat.variables[selected_vars[index1] as usize].options[selection as usize];
		
		println!("For: \"{}\", you picked: \"{}\"", selected_cat.variables[selected_vars[index1] as usize].name, selected_option);
		variables_to_filter.push(
			Variable_to_filter {
				id_name: format!("{}/ {}", selected_cat.variables[selected_vars[index1] as usize].id, selected_cat.variables[selected_vars[index1] as usize].name),
				selected_option_id_name: selected_option .to_string()
			}
		);
	}
	
	println!("========");
	println!("Recap:");
	println!("Category: {:?}", selected_cat.name);
	println!("{:?}", variables_to_filter);
	println!("========");
	//let userId = "8w12903x"; //VANCANTORUS
	//let guest_name = "[]vancantorus"; //VANCANTORUS
	//let gameId = "kdkq49xd"; //VS Homer
	
	
	
	let all_runs = fetch_all_runs(userId, gameId);
	let mut il_runs: Vec<JsonValue> = Vec::new();
	for run in all_runs {
		if !run["level"].is_null() {
			il_runs.push(run);
		}
	}
	println!("========");
	println!("IL runs: {}", il_runs.len());
	
	let mut filtered_il_runs: Vec<JsonValue> = Vec::new();
	for run in il_runs {
		let run_values = &run["values"];
		let mut all_match = true;
		for variable in &variables_to_filter {
			if run_values[ &variable.id_name[0..8] ].as_str() != Some(&variable.selected_option_id_name[0..8]) {
				all_match = false;
				break;
			}
		}
		
		if all_match {
			if run["category"] == selected_cat.id{
				filtered_il_runs.push(run.clone());
			}
		}
	}
	println!("Runs with selected variables options: {}", filtered_il_runs.len());

	for run in filtered_il_runs {
		println!("{}", run["id"]);
		
		let players = &run["players"];
		println!("Old player list:");
		let mut players_as_string = String::new();
		players_as_string.push_str("{ \"players\": [\n");
		
		let mut should_comma_next_left = false;
		for player in players.members() {
			if should_comma_next_left {
				players_as_string.push_str(",\n");
			}
			if !player["id"].is_null() /*if not guest*/ {
				players_as_string.push_str(&format!("    {{\"rel\": \"{}\", \"id\": \"{}\"}}", player["rel"], player["id"]));
			} else {
				players_as_string.push_str(&format!("    {{\"rel\": \"{}\", \"name\": \"{}\"}}", player["rel"], player["name"]));
				//players_as_string.push_str(&format!("{} {}\n", player["rel"], player["name"]));
			}
			should_comma_next_left = true;
			//println!("{} {}", player["rel"], if player["id"].is_null() { &player["name"] } else { &player["id"] });
		}
		players_as_string.push_str("\n  ]\n}");
		
		println!("{}", players_as_string);
		println!("--------");
		
		println!("New player list:");
		let mut players_as_string2 = players_as_string.replace(
			&format!("    {{\"rel\": \"user\", \"id\": \"{}\"}}", userId),
			&format!("    {{\"rel\": \"guest\", \"name\": \"{}\"}}", guest_name)
		);
		
		
		println!("{}", players_as_string2);
		println!("========");
		
		let output = Command::new("curl")
			.arg("-L")
			.arg("-X")
			.arg("PUT")
			.arg(&format!("https://www.speedrun.com/api/v1/runs/{}/players", run["id"]))
			.arg("-H")
			.arg("Content-Type: application/json")
			.arg("-H")
			.arg(&format!("X-Api-Key: {}", api_key))
			.arg("--data-raw")
			.arg(players_as_string2)		
			.output()
		.expect("Failure");
				
		let string_to_print = String::from_utf8_lossy(&output.stdout);
		println!("{:?}", string_to_print);
		
		sleep(Duration::from_millis(to_sleep));
	}
	
	println!("==================\nDone");
}
