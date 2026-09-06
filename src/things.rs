use std::io;
use std::collections::HashSet;
use serde::{Deserialize};
use chrono::Datelike;
use chrono::Weekday;

pub fn check_update() -> Result<(), Box<dyn std::error::Error>> {
	let this_version = env!("CARGO_PKG_VERSION");
	
	let response = ureq::get("https://api.github.com/repos/Afdusrt/SRCutils/releases/latest").call();

    let mut resp_1 = response?;

    let body_string = resp_1.body_mut().read_to_string()?;
    
    let json_body = json::parse(&body_string)?;
    
    if this_version == json_body["tag_name"] {
		println!("no updates needed");
	} else {
		let url = json_body["html_url"].as_str().unwrap();

		println!("Update available: {url}");
	}
	
    Ok(())
}

pub fn ask_bool(question: &str) -> bool {
	loop {
        println!("{question} (y/n)");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        match answer.trim() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => println!("Enter 'y' or 'n' baka"),
        }
    }
}

pub fn get_user_leaderboard(user_id: &str) -> Result<String, ureq::Error> {
	println!("Fetching GetUserLeaderboard for: {}", user_id);

    let url = format!(
        "https://www.speedrun.com/api/v2/GetUserLeaderboard?userId={}",
        user_id
    );

    let response = ureq::get(url).call();

    let mut resp_1 = match response {
        //read_to_string needs mut body
        Ok(response_body) => response_body,
        Err(e) => {
            /*eprintln!("Failed fetch: {}", e);*/
            return Err(e);
        }
    };

    match resp_1.body_mut().read_to_string() {
        //read_to_string needs mut body
        Ok(body_string) => return Ok(body_string),
        Err(e) => {
            /*eprintln!("Failed read to string: {}", e);*/
            return Err(e);
        }
    };
}

pub fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
	let mut response = ureq::get(url).call()?;

    Ok( response.body_mut().read_to_string()? )
}

#[derive(Debug, Deserialize)]
pub struct LeaderboardResponse {
    pub user: UserFields,
    pub categories: serde_json::Value,
    pub levels: serde_json::Value,
    pub platforms: serde_json::Value,
    pub runs: Vec<Run>,
}

#[derive(Debug, Deserialize)]
pub struct UserFields {
    pub name: String,
    pub areaId: String,
}

#[derive(Debug, Deserialize)]
pub struct Run {
    gameId: String,
    categoryId: Option<String>,
    place: Option<u32>,
    levelId: Option<String>,
    platformId: Option<String>,
    date: Option<i64>,
    obsolete: Option<bool>,
    orphaned: Option<bool>,
    playerIds: Vec<String>,
}

#[derive(Debug, Default)]
pub struct User {
    pub name: String,
    pub areaId: String,
    pub lbs: Vec<Leaderboard>
}

#[derive(Debug)]
pub enum LeaderboardValue {
    Count(usize),
    HashSet( (HashSet<String>, fn(&Run, &mut HashSet<String>) ) ),
    ArrayLen( (usize, fn(&LeaderboardResponse) -> usize) ),
}

impl LeaderboardValue {
    pub fn score(&self) -> usize {
        match self {
            LeaderboardValue::Count(count) => *count,
            LeaderboardValue::ArrayLen((count, _)) => *count,
            LeaderboardValue::HashSet((set, _)) => set.len(),
        }
    }
}

#[derive(Debug)]
pub struct Leaderboard {
	pub name: &'static str,
	pub lb_size: usize,
	pub run_filter: fn(&Run) -> bool,
	pub value: LeaderboardValue
}

fn is_weekday(timestamp_pre: Option<i64>, weekday: Weekday) -> bool {
	let timestamp = match timestamp_pre {
		Some(res) => res,
		None => return false
	};
	
	if let Some(date) = chrono::DateTime::from_timestamp(timestamp, 0) {
		let date_weekday = date.weekday();
		if date_weekday == weekday {
			return true
		}
		//return true
	}
	
	false
}

pub fn leaderboards_vec() -> Vec<Leaderboard> {
	vec![
		Leaderboard {
			name: "wr-leaderboards",
			lb_size: 200,
			run_filter: |run| { if run.place == Some(1) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "runs-leaderboards",
			lb_size: 200,
			run_filter: |_| true,
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "games-leaderboards",
			lb_size: 200,
			run_filter: |_| true,
			value: LeaderboardValue::HashSet((
				HashSet::new(),
				|run, set| {
					set.insert(run.gameId.clone());
				},
			)),
		},
		Leaderboard {
			name: "categories-leaderboards",
			lb_size: 200,
			run_filter: |_| false,
			value: LeaderboardValue::ArrayLen( ( 0, |data: &LeaderboardResponse| data.categories.as_array().unwrap().len() ) )
		},
		Leaderboard {
			name: "wrs-full-game-leaderboards",
			lb_size: 200,
			run_filter: |run| { if run.place == Some(1) && run.levelId == None { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "wrs-il-leaderboards",
			lb_size: 200,
			run_filter: |run| { if run.place == Some(1) && !(run.levelId == None) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "runs-il-leaderboards",
			lb_size: 200,
			run_filter: |run| { if !(run.levelId == None) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "runs-full-game-leaderboards",
			lb_size: 200,
			run_filter: |run| { if run.levelId == None { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "podiums-leaderboards",
			lb_size: 200,
			run_filter: |run| { if run.place == Some(1) || run.place == Some(2) || run.place == Some(3) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "games-with-wrs-leaderboards",
			lb_size: 200,
			run_filter: |run| { if run.place == Some(1) { true } else { false } },
			value: LeaderboardValue::HashSet((
				HashSet::new(),
				|run, set| {
					set.insert(run.gameId.clone());
				},
			)),
		},
		Leaderboard {
			name: "obsoletes-leaderboards",
			lb_size: 200,
			run_filter: |run| { if run.obsolete == Some(true) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "personal-bests",
			lb_size: 200,
			run_filter: |run| { if !(run.obsolete == Some(true)) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "levels",
			lb_size: 200,
			run_filter: |_| false,
			value: LeaderboardValue::ArrayLen( (0, |data: &LeaderboardResponse| data.levels.as_array().unwrap().len() ) )
		},
		Leaderboard {
			name: "co-op-partners-leaderboards",
			lb_size: 50,
			run_filter: |_| true,
			value: LeaderboardValue::HashSet((
				HashSet::new(),
				|run, set| {
					if run.playerIds.len() > 1 {
                        for playerid in &run.playerIds {
							if !(playerid.len() > 8) {
								set.insert(playerid.clone());
							}
                        }
                    }
				},
			)),
		},
		Leaderboard {
			name: "platforms",
			lb_size: 50,
			run_filter: |_| false,
			value: LeaderboardValue::ArrayLen( (0, |data: &LeaderboardResponse| data.platforms.as_array().unwrap().len() ) )
		},
		Leaderboard {
			name: "orphaned-runs",
			lb_size: 50,
			run_filter: |run| { if run.orphaned == Some(true) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "platforms-solo",
			lb_size: 50,
			run_filter: |run| { if run.playerIds.len() > 1 { false } else { true } },
			value: LeaderboardValue::HashSet((
				HashSet::new(),
				|run, set| {
					if let Some(platformId) = &run.platformId {
						set.insert(platformId.to_string());
					}
				},
			)),
		},
		Leaderboard {
			name: "categories-fg",
			lb_size: 200,
			run_filter: |run| { if run.levelId == None { true } else { false } },
			value: LeaderboardValue::HashSet((
				HashSet::new(),
				|run, set| {
					if let Some(categoryId) = &run.categoryId {
						set.insert(categoryId.to_string());
					}
				},
			)),
		},
		Leaderboard {
			name: "speedruns-solo",
			lb_size: 200,
			run_filter: |run| { if run.playerIds.len() > 1 { false } else { true } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "wrs-solo",
			lb_size: 200,
			run_filter: |run| { if (run.playerIds.len() == 1) && (run.place == Some(1)) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-coop-fg",
			lb_size: 200,
			run_filter: |run| { if run.levelId == None && run.playerIds.len() > 1 { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedrunsruns-coop-il",
			lb_size: 200,
			run_filter: |run| { if !(run.levelId == None) && run.playerIds.len() > 1 { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "wrs-coop",
			lb_size: 200,
			run_filter: |run| { if (run.playerIds.len() > 1) && (run.place == Some(1)) { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "games-fg-only",
			lb_size: 200,
			run_filter: |run| { if run.levelId == None { true } else { false } },
			value: LeaderboardValue::HashSet((
				HashSet::new(),
				|run, set| {
					set.insert(run.gameId.clone());
				},
			)),
		},
		Leaderboard {
			name: "orphaned-fg-speedruns",
			lb_size: 50,
			run_filter: |run| { if run.orphaned == Some(true) && run.levelId == None { true } else { false } },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-monday",
			lb_size: 100,
			run_filter: |run| { is_weekday(run.date, Weekday::Mon) },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-tuesday",
			lb_size: 100,
			run_filter: |run| { is_weekday(run.date, Weekday::Tue) },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-wednesday",
			lb_size: 100,
			run_filter: |run| { is_weekday(run.date, Weekday::Wed) },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-thursday",
			lb_size: 100,
			run_filter: |run| { is_weekday(run.date, Weekday::Thu) },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-friday",
			lb_size: 100,
			run_filter: |run| { is_weekday(run.date, Weekday::Fri) },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-saturday",
			lb_size: 100,
			run_filter: |run| { is_weekday(run.date, Weekday::Sat) },
			value: LeaderboardValue::Count(0),
		},
		Leaderboard {
			name: "speedruns-sunday",
			lb_size: 100,
			run_filter: |run| { is_weekday(run.date, Weekday::Sun) },
			value: LeaderboardValue::Count(0),
		},
	]
}

pub fn compute_user(data: LeaderboardResponse) -> User {
	let mut lbs = leaderboards_vec();
	
	for run in &data.runs {
		for lb in &mut lbs {
            if !(lb.run_filter)(run) {
                continue;
            }
            
            match &mut lb.value {
                LeaderboardValue::Count(count) => {
                    *count += 1;
                },
                LeaderboardValue::HashSet((set, insert_fn)) => {
					insert_fn(run, set);
                },
				_ => {}
            }
		}
	}
	
	for lb in &mut lbs {
		match &mut lb.value {
			LeaderboardValue::ArrayLen((count, get_len)) => {
				*count = get_len(&data);
			},
			_ => {}
		}
	}
	
	User {
        name: data.user.name,
        areaId: data.user.areaId,
        lbs,
    }
}
