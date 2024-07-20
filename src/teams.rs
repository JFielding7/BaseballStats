use serde::Deserialize;
use reqwest::blocking::get;
use crate::data_id::get_id;
use crate::hitting_stats::BasicStatistics;

#[derive(Deserialize)]
struct Roster {
    players: Vec<Player>
}

#[derive(Deserialize)]
struct Player {
    person: Person,
    position: Position
}

#[derive(Deserialize)]
struct Person {
    id: i32
}

#[derive(Deserialize)]
struct Position {
    abbreviation: String
}

// fn get_player_hitting_stats(team_id: i32) -> Vec<BasicStatistics> {
//     let roster: Roster = get(format!("https://statsapi.mlb.com/api/v1/teams/{}/roster?rosterType=fullSeason", team_id))
//         .unwrap().json().unwrap();
//     let hitters = &roster.players.iter().map(|player: Player| {
//         let stats: BasicStatistics = get(format!("https://statsapi.mlb.com/api/v1/people/{}/stats?stats=season&group=hitting", player.person.id))
//             .unwrap().json().unwrap();
//         stats
//
//     }).collect();
// }
//
// pub(crate) fn display_team_stats(query: &Vec<String>) {
//     const ID_LEN: usize = 3;
//     let team = &query[1];
//
//     let team_id = get_id("database/team_ids.txt", team, ID_LEN).unwrap();
//     if team_id.is_positive() {
//         get_player_hitting_stats(team_id);
//     }
//     else {
//         println!("Invalid Team!")
//     }
// }