use std::collections::HashMap;
use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::hitting_stats::{Batter};
use crate::pitching_stats::{Pitcher};
use crate::stats;

#[derive(Deserialize)]
struct BoxScore {
    teams: Teams
}

#[derive(Deserialize)]
struct Teams {
    away: Team,
    home: Team,
}

#[derive(Deserialize)]
struct Team {
    team: TeamName,
    players: HashMap<String, Player>
}

#[derive(Deserialize)]
struct Player {
    person: stats::Player,
    stats: Stats
}

#[derive(Deserialize)]
struct TeamName {
    abbreviation: String
}

#[derive(Deserialize)]
struct Stats {
    #[serde(deserialize_with = "deserialize_stats")]
    batting: Option<Batter>,
    #[serde(deserialize_with = "deserialize_stats")]
    pitching: Option<Pitcher>
}

fn deserialize_stats<'de, S, D>(deserializer: D) -> Result<Option<S>, D::Error>
where S: serde::Deserialize<'de>, D: serde::Deserializer<'de> {
    Ok(Option::<S>::deserialize(deserializer).unwrap_or(None))
}

macro_rules! box_score_url {
    ($game_id:expr) => { format!("https://statsapi.mlb.com/api/v1/game/{}/boxscore", $game_id) };
}

pub(crate) fn display_game_stats(game_id: i32) {
    let box_score: BoxScore = get(box_score_url!(game_id)).unwrap().json().unwrap();
    println!("{:?}", box_score.teams.away.team.abbreviation);
}

