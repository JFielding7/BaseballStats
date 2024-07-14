use std::collections::HashMap;
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
}

const MLB_START_SEASON: i32 = 1876;

fn get_players(all_time: bool) -> HashMap<String, i32> {
    let current_season = Utc::now().year();
    let mut start_season = current_season;
    if all_time { start_season = MLB_START_SEASON }

    let mut baseball_players: HashMap<String, i32> = HashMap::new();
    for season in start_season..(current_season + 1) {
        let players: Players = reqwest::blocking::get(format!("https://statsapi.mlb.com/api/v1/sports/1/players?season={}", season))
            .unwrap().json().unwrap();
        for player in &players.people {
            let name_key = &player.nameSlug;
            baseball_players.insert(name_key[..name_key.rfind("-").unwrap()].to_string(), player.id);
        }
        // println!("{} {}", baseball_players.len(), players.people.len());
    }
    return baseball_players;
}

pub(crate) fn update_players(all_time: bool) {
    let players = get_players(all_time);
    let mut sorted_players: Vec<(&String, &i32)> = Vec::with_capacity(players.len());
    players.iter().for_each(|player| sorted_players.push(player));
    sorted_players.sort();

    let player_file = File::create("database/players.txt").expect("Could not create file.");
    let mut player_writer = LineWriter::new(player_file);

    let id_file = File::create("database/ids.txt").expect("Could not create file.");
    let mut id_writer = LineWriter::new(id_file);

    for (name, id) in &sorted_players {
        writeln!(player_writer, "{}", name).expect("Could not write to file.");
        writeln!(id_writer, "{}", id).expect("Could not write to file.");
    }
    player_writer.flush().expect("Error flushing buffer.");
}