use std::cmp::max;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{LineWriter, Write, Result};
use serde::Deserialize;
use chrono::{Datelike, Utc};
use reqwest::blocking::{get};

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
        let players: Players = get(url).unwrap().json().unwrap();
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

pub(crate) fn update_players(all_time: bool) -> Result<()> {
    let baseball_players = get_players(all_time);
    let mut sorted_players: Vec<(String, bool, &i32)> = Vec::with_capacity(baseball_players.len());

    let mut max_len = 0;
    for (name, players) in &baseball_players {
        let mut i = 0;
        let is_multiple_players = players.len() > 1;
        for (player_id, position) in players {
            let mut distinct_name;
            if is_multiple_players {
                distinct_name = format!("{}-{}-{}", name.clone(), i, position);
            }
            else {
                distinct_name = name.clone();
            }
            max_len = max(max_len, distinct_name.len());
            sorted_players.push((distinct_name, position == "P", player_id));
            i += 1;
        }
    }
    sorted_players.sort();

    let player_id_file = File::create("database/player_ids.txt")?;
    let mut player_id_writer = LineWriter::new(player_id_file);

    let player_file = File::create("database/players.txt")?;
    let mut player_writer = LineWriter::new(player_file);

    for (name, is_pitcher, id) in sorted_players {
        writeln!(player_id_writer, "{} {}{} {}", name, " ".repeat(max_len - name.len()), is_pitcher as u8, id)?;
        writeln!(player_writer, "{}", name)?;
    }
    player_id_writer.flush()?;
    player_writer.flush()?;

    Ok(())
}


#[derive(Deserialize)]
struct Teams {
    teams: Vec<Team>
}

#[derive(Deserialize)]
struct Team {
    id: i32,
    fileCode: String
}


pub(crate) fn update_teams() -> Result<()> {
    let url = "https://statsapi.mlb.com/api/v1/teams?sportId=1";
    let mut teams: Teams = get(url).unwrap().json().unwrap();

    let team_id_file = File::create("database/team_ids.txt")?;
    let mut team_id_writer = LineWriter::new(team_id_file);

    let team_file = File::create("database/teams.txt")?;
    let mut team_writer = LineWriter::new(team_file);

    let mut max_len = 0;
    for team in &teams.teams {
        max_len = max(max_len, team.fileCode.len());
    }
    teams.teams.sort_by(|team0, team1| team0.fileCode.cmp(&team1.fileCode));

    for team in &teams.teams {
        writeln!(team_id_writer, "{}{}{}", &team.fileCode, " ".repeat(max_len - team.fileCode.len() + 1), team.id)?;
        writeln!(team_writer, "{}", &team.fileCode)?;
    }
    team_id_writer.flush()?;
    team_writer.flush()?;

    Ok(())
}

pub(crate) fn update_database(all_time: bool) -> Result<()>  {
    update_players(all_time)?;
    update_teams()?;
    Ok(())
}