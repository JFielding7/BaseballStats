use std::cmp::max;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{LineWriter, Write};
use serde::Deserialize;
use chrono::{Datelike, Utc};

#[derive(Deserialize)]
struct Players {
    people: Vec<Player>
}

#[derive(Deserialize)]
struct Player {
    id: i32,
    nameSlug: String,
    primaryPosition: Position
}

#[derive(Deserialize)]
struct Position {
    abbreviation: String
}

const START_SEASON: i32 = 1876;

fn get_players(all_time: bool) -> HashMap<String, HashMap<i32, String>> {
    let current_season = Utc::now().year();
    let mut start_season = current_season;
    if all_time { start_season = START_SEASON }

    let mut baseball_players: HashMap<String, HashMap<i32, String>> = HashMap::new();

    for season in start_season..(current_season + 1) {
        let url = format!("https://statsapi.mlb.com/api/v1/sports/1/players?season={}", season);
        let players: Players = reqwest::blocking::get(url).unwrap().json().unwrap();
        for player in &players.people {
            let name_key = &player.nameSlug;
            let name = name_key[..name_key.rfind("-").unwrap()].to_string();

            let players = baseball_players.get_mut(&name);
            if players.is_none() {
                baseball_players.insert(name, HashMap::from([(player.id, player.primaryPosition.abbreviation.clone())]));
            }
            else {
                players.unwrap().insert(player.id, player.primaryPosition.abbreviation.clone());
            }
        }
    }
    baseball_players
}

pub(crate) fn update_players(all_time: bool) {
    let baseball_players = get_players(all_time);
    let mut sorted_players: Vec<(String, &i32)> = Vec::with_capacity(baseball_players.len());

    let mut max_len = 0;
    for (name, players) in &baseball_players {
        let mut i = 0;
        let is_multiple_players = players.len() > 1;
        for (player_id, position) in players {
            let mut distinct_name;
            if is_multiple_players { distinct_name = format!("{}-{}-{}", name.clone(), i, position); }
            else { distinct_name = name.clone(); }
            max_len = max(max_len, distinct_name.len());
            sorted_players.push((distinct_name, player_id));
            i += 1;
        }
    }
    sorted_players.sort();

    let player_id_file = File::create("database/player_ids.txt").expect("Could not create file.");
    let mut player_id_writer = LineWriter::new(player_id_file);

    let player_file = File::create("database/players.txt").expect("Could not create file.");
    let mut player_writer = LineWriter::new(player_file);

    for (name, id) in &sorted_players {
        writeln!(player_id_writer, "{}{}{}", name, " ".repeat(max_len - name.len() + 1), id).expect("Could not write to file.");
        writeln!(player_writer, "{}", name).expect("Could not write to file.");
    }
    player_id_writer.flush().expect("Error flushing buffer.");
}