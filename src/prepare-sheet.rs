use std::{ env, fs, io, process, process::Command, };

fn fetch(s: &str, n: &str) -> String {
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

fn largest_options_count(category: &Category) -> usize {
    category.variables
        .iter()
        .map(|var| var.options.len())  // convert each variable to its option count
        .max()                         // take the largest
        .unwrap_or(0)                  // 0 if no variables
}

// Example function to write the CSV to a file
fn write_csv(matrix: Vec<Vec<String>>, filename: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename)?;
    for row in matrix {
        writeln!(file, "{}", row.join("|"))?;
    }
    Ok(())
}

const HELP: &str = "HELP:
====
arg 1 - game abbreviation
arg 2 - csv file to save
=
You will be prompted things.
You will need to open the csv file in a program like libreoffice, to fill out correct values before using submit-sheet, refer to the README.
";

fn main() {
	let args: Vec<_> = env::args().collect();
	
	if args.len() < 3 {
		eprintln!("{}", HELP);
		process::exit(1);
	}
	
	let game_abbreviation = &args[1];
	let file_to_save = &args[2];
	let modifier = &args[args.len()-1];
	
	let categories_raw = fetch(&format!("v1/games/{}/categories", game_abbreviation), "categories");
	let parsed_categories = json::parse(&categories_raw).unwrap();
	
	let mut cats: Vec<Category> = Vec::new();
	
	println!("Pick category?:\n========"); //prints categories, their id, and scope
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
    println!("You picked category: {}", cats[selection as usize].name);
    let mut selected_cat = &mut cats[selection as usize];
    
    /*let mut is_level = false;
    if cats[selection as usize].kind == "per-level" {
		will_need_to_do_level_shit = true
	}*/
    
    //let variables_raw = fetch(&format!("categories/{}/variables", cats[selection as usize].id), "variables");
    let variables_raw = fetch(&format!("v1/categories/{}/variables", selected_cat.id), "variables");
    let variables2_raw = fetch(&format!("v1/games/{}/variables", game_abbreviation), "variables2");
    let parsed_variables = json::parse(&variables_raw).unwrap();
    
    let mut vars: Vec<Variable> = Vec::new();
    let mut vars_per_level: Vec<Variable> = Vec::new();
    
    //println!("Variables for {}:\n========", cats[selection as usize].name);
    println!("Variables for {}:\n========", selected_cat.name);
    for i in 0..parsed_variables["data"].len() {
		//variable==== id/ name
		println!("{}/ {}", parsed_variables["data"][i]["id"], parsed_variables["data"][i]["name"]);
		
		let mut is_per_level = false;
		let mut if_il_id = String::new();
		
		if parsed_variables["data"][i]["scope"]["type"] == "single-level" {
			if_il_id = parsed_variables["data"][i]["scope"]["level"].to_string();
			is_per_level = true;
		}
		
		let mut options: Vec<String> = Vec::new();
		//varible options
		for (id, name) in parsed_variables["data"][i]["values"]["values"].entries() {
			let option = format!("{}/ {}", id, name["label"]);
			//println!("===={}/ {}", id, name["label"]);
			options.push(option);
		}
		
		//variable==== options Vec<optionID,optionNAME
		//println!("{:?}", options);
		if is_per_level {
			vars_per_level.push( Variable {
					id: parsed_variables["data"][i]["id"].to_string(),
					name: parsed_variables["data"][i]["name"].to_string(),
					if_il_id: if_il_id,
					options: options, //Vec<String>
			});
		} else {
			vars.push( Variable {
					id: parsed_variables["data"][i]["id"].to_string(),
					name: parsed_variables["data"][i]["name"].to_string(),
					if_il_id: if_il_id,
					options: options, //Vec<String>
			});
		}
		//println!("{}/ {} |Options:", vars[i].id, vars[i].name);
	}
	
	//cats[selection as usize].variables = vars;
	selected_cat.variables = vars;
	selected_cat.variables_per_level = vars_per_level;
	/*
#[derive(Debug)]
struct Variable {
	id: String,
	name: String,
	//is_per_level: bool,
	//if_is_per_level_id: String,
	options: Vec<String>,
}
#[derive(Debug)]
struct Category {
	id: String,
	name: String,
	is_il: bool,
	variables: Vec<Variable>,
	variables_per_level: Vec<Variable>
}*/
	println!("======== recap");
	println!("Picked category (id, name, is_il):");
	println!();
	println!("={}/ {}/ {}", selected_cat.id, selected_cat.name, selected_cat.is_il);
	println!("This category has these variables, and these options:");
	for i in 0..selected_cat.variables.len() {
		println!("{}/ {}/", selected_cat.variables[i].id,
									selected_cat.variables[i].name,
		);
		println!("={:?}", selected_cat.variables[i].options);
	}
	println!("Category also has {} per-level variables", selected_cat.variables_per_level.len());
	
	//====================
	//====================
	//====================
	//====================
	//====================
	println!("=========\n Now, select a platform for the game:");
	//https://www.speedrun.com/api/v2/GetGameData?gameUrl=color_book
	let ggd_raw = fetch(&format!("v2/GetGameData?gameUrl={}", game_abbreviation), "ggd");
    let parsed_ggd = json::parse(&ggd_raw).unwrap();
    
    let mut plats: Vec<String> = Vec::new();
	plats.push("NO".to_string());
    for i in 0..parsed_ggd["platforms"].len() {
		plats.push( format!("{}/ {}", parsed_ggd["platforms"][i]["id"], parsed_ggd["platforms"][i]["name"]));
		//println!("{}/ {}/ {}", i, parsed_ggd["platforms"][i]["id"], parsed_ggd["platforms"][i]["name"]);
	}
	//for i in 0..=parsed_ggd["platforms"].len() {
	for i in 0..plats.len() {
		//println!("{}/ {}/ {}", i, parsed_ggd["platforms"][i]["id"], parsed_ggd["platforms"][i]["name"]);
		println!("{}/ {}", i, plats[i]);
	}
	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let selection: i32 = input_text.trim().parse().expect("string was not intable");
    
    let mut selected_plat = &plats[selection as usize];
    println!("{}", selected_plat);
    //====================
    
    //====================
    println!("=========\n Now, select a Region for the game (or 'NO':");
    let mut regions: Vec<String> = Vec::new();
    regions.push("NO".to_string());
    
    for i in 0..parsed_ggd["regions"].len() {
		regions.push( format!("{}/ {}", parsed_ggd["regions"][i]["id"], parsed_ggd["regions"][i]["name"]));
		//println!("{}/ {}/ {}", i, parsed_ggd["regions"][i]["id"], parsed_ggd["regions"][i]["name"]);
	}
	for i in 0..=parsed_ggd["regions"].len() {
		println!("{}/ {}", i, regions[i]);
	}
	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let input_text_trimmed = input_text.trim();
    
    let mut selection: i32 = 0;
    let mut selected_region = &regions[0];
    if input_text_trimmed.to_lowercase() == "no" {
		selected_region = &"NO".to_string();
	} else {
		selection = input_text_trimmed.parse().expect("string was not intable");;
	}
    let mut selected_region = &regions[selection as usize];
    
    println!("{}", selected_region);
	/*
	println!("Pick category?:\n========"); //prints categories, their id, and scope
		for i in 0..parsed_categories["data"].len() {
			cats.push( Category {
				id: parsed_categories["data"][i]["id"].to_string(),
				name: parsed_categories["data"][i]["name"].to_string(),
			});
		}
		
		for i in 0..cats.len() {
			println!("{}/ {}/ {}/ {}", i, cats[i].id, cats[i].name, cats[i].is_il);
		}
	
	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let selection: i32 = input_text.trim().parse().expect("string was not intable");
    println!("You picked category: {}", cats[selection as usize].name);
    let mut selected_cat = &mut cats[selection as usize];
    */
    
	let mut max_options = largest_options_count(&selected_cat);
	max_options += 2;
	println!("Maximum options for any variable: {}", max_options);
	
	let mut matrix: Vec<Vec<String>> = vec![];
	
	let num_columns = 11 + (selected_cat.variables.len() * 2);

	for _ in 0..(max_options + 3)+1 {
		let row = vec!["".to_string(); num_columns]; // empty strings for now
		matrix.push(row);
	}
	
	matrix[0][0] = "category:".to_string();
	matrix[1][0] = format!("{}/ {}", selected_cat.id, selected_cat.name);
	matrix[2][0] = format!("{}", selected_cat.is_il);
	matrix[3][0] = format!("{}", max_options+4);
	
	for i in 0..selected_cat.variables.len() {
		let mut n = i*2;
		n += 9;
		matrix[0][n+2] = "Variable:".to_string();
		matrix[0][n+3] = "Options:".to_string();
		matrix[1][n+2] = format!("{}/ {}", selected_cat.variables[i].id, selected_cat.variables[i].name);
		matrix[max_options+3][n+2] = format!("{}/ {}", selected_cat.variables[i].id, selected_cat.variables[i].name);
		//matrix[0][n+1] = selected_cat.variables[i].name	.clone();
		for e in 0..selected_cat.variables[i].options.len() {
			matrix[e+1][n+3] = selected_cat.variables[i].options[e]	.clone();
		}
	}
	
	matrix[max_options+3][0] = "NO".to_string();
	
	matrix[max_options+2][0] = "date".to_string();
		matrix[max_options+3][0] = "NO".to_string();
	matrix[max_options+2][1] = "region".to_string();
		matrix[max_options+3][1] = selected_region.to_string();
	matrix[max_options+2][2] = "platform".to_string();
		matrix[max_options+3][2] = selected_plat.to_string();
	matrix[max_options+2][3] = "emulated".to_string();
		matrix[max_options+3][3] = "NO".to_string();
	matrix[max_options+2][4] = "players".to_string();
		matrix[max_options+3][4] = "NO".to_string();
	matrix[max_options+2][5] = "level".to_string();
		matrix[max_options+3][5] = "NO".to_string();
	matrix[max_options+2][6] = "LRT".to_string();
		matrix[max_options+3][6] = "NO".to_string();
	matrix[max_options+2][7] = "RTA".to_string();
	matrix[max_options+2][8] = "IGT".to_string();
	matrix[max_options+2][9] = "VIDEO".to_string();
	matrix[max_options+2][10] = "COMMENT".to_string();


	let mut levels: Vec<String> = Vec::new();
	
	//parsed_ggd
	if selected_cat.is_il {
		for i in 0..parsed_ggd["levels"].len() {
			levels.push( format!("{}/ {}", parsed_ggd["levels"][i]["id"], parsed_ggd["levels"][i]["name"]) )
		}
		for i in 0..=parsed_ggd["regions"].len() {
			println!("{}/ {}", i, regions[i]);
		}
		println!("{:?}", levels);
		
		for _ in 0..levels.len()-1 {
			let row = vec!["".to_string(); num_columns]; // empty strings for now
			matrix.push(row);
		}

		for i in 0..levels.len() {
			matrix[max_options+3+i][5] = levels[i]	.clone();
		}
		
		for row in (max_options + 3)..matrix.len() {
			let level_cell = &matrix[row][5].clone();

			if level_cell.len() < 8 {
				continue;
			}

			let level_id = &level_cell[..8];

			for var in &selected_cat.variables_per_level {
				if var.if_il_id == level_id {
					matrix[row].push( format!("{}/ {}", var.id, var.name) );
					matrix[row].push( format!("{:?}", var.options) );
					//println!("Matched variable {} for level {}", var.name, level_id);
				}
			}
		}
	}
	//====================
	println!("{:?}", matrix);
	println!();
	println!();
	println!();
	println!();
	println!("{:?}", selected_cat.variables_per_level);
	write_csv(matrix, file_to_save);
	//to be csv file
	/*
	selected_cat.id/name|
		//input "NO" if you dont include the thing in the payload (date, region, players, LRT, comment, splitsio)
	ig just put 10 indeces here and then start the reference thingy?	 | selected_cat.variables[0] | selected_cat.variables[0].options[0] | selected_cat.variables[1] | selected_cat.variables[1].options[0]
																									 | selected_cat.variables[0].options[1] |                           | selected_cat.variables[1].options[1]
		//guaranteed 10 indeces for universal thingy
	0	 1		2		 3		  4		  5	    6	7	8	9	  10	  	11
	date|region|platform|emulated|players|level|LRT|RTA|IGT|VIDEO|COMMENT|	selected_cat.variables[0]| - 									|selected_cat.variables[1]  | - |...
	
	*/
	
}
