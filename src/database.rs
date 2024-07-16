use std::collections::{HashMap, HashSet};
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
    currentTeam: Team
}

#[derive(Deserialize)]
struct Team {
    id: i32
}

const MLB_START_SEASON: i32 = 2023;

fn get_distinct_name(player: &Player, players: &mut HashMap<String, i32>, duplicates: &mut HashMap<String, HashSet<i32>>) {
    let name_key = &player.nameSlug;
    let name = name_key[..name_key.rfind("-").unwrap()].to_string();

    let id_option = players.get(&name);
    if id_option.is_none() {
        players.insert(name, player.id);
    }
    else if *id_option.unwrap() != player.id {
        let dup_name_ids = duplicates.entry(name.clone()).or_insert(HashSet::new());
        dup_name_ids.insert(player.id);
        let distinct_name = format!("{}-{}", name, dup_name_ids.len());
        players.insert(distinct_name, player.id);
    }
}

fn get_players(all_time: bool) -> HashMap<String, i32> {
    let current_season = Utc::now().year();
    let mut start_season = current_season;
    if all_time { start_season = MLB_START_SEASON }

    let mut baseball_players: HashMap<String, i32> = HashMap::new();
    let mut duplicates: HashMap<String, HashSet<i32>> = HashMap::new();

    for season in start_season..(current_season + 1) {
        let players: Players = reqwest::blocking::get(format!("https://statsapi.mlb.com/api/v1/sports/1/players?season={}", season))
            .unwrap().json().unwrap();
        for player in &players.people {
            get_distinct_name(player, &mut baseball_players, &mut duplicates);
        }
        println!("{} {}", players.people.len(), baseball_players.len());
    }
    baseball_players
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