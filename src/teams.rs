use serde::Deserialize;
use reqwest::blocking::get;
use crate::data_id::get_id;

#[derive(Deserialize)]
struct Roster {
    players: Vec<Player>
}

#[derive(Deserialize)]
struct Player {
    person: Person
}

#[derive(Deserialize)]
struct Person {
    id: i32
}

fn display_stats(team_id: i32) {
    let roster: Roster = get(format!("https://statsapi.mlb.com/api/v1/teams/{}/roster?rosterType=fullSeason", team_id)).unwrap().json().unwrap();
    for player in &roster.players {

    }
}

pub(crate) fn display_team_stats(query: &Vec<String>) {
    const ID_LEN: usize = 3;
    let team = &query[1];

    let team_id = get_id("database/team_ids.txt", team, ID_LEN).unwrap();
    if team_id.is_positive() {
        display_stats(team_id);
    }
    else {
        println!("Invalid Team!")
    }
}