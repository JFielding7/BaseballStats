use serde::Deserialize;
use reqwest::blocking::get;
use crate::data_id::get_id;
use crate::hitting_stats::BasicStatistics;

#[derive(Deserialize)]
struct Roster {
    roster: Vec<Player>
}

#[derive(Deserialize)]
struct Player {
    person: Person,
    position: Position
}

#[derive(Deserialize)]
struct Person {
    id: i32,
    fullName: String
}

#[derive(Deserialize)]
struct Position {
    abbreviation: String
}

const PITCHER: &str = "P";

macro_rules! roster_url {
    () => { "https://statsapi.mlb.com/api/v1/teams/{}/roster?rosterType=fullSeason" };
}

macro_rules! hitting_stats_url {
    () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats=season&group=hitting" };
}

macro_rules! pitching_stats_url {
    () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats=season&group=pitching" };
}

fn get_team_hitting_stats(roster: &Roster) -> Vec<BasicStatistics> {
    let mut hitter_stats: Vec<BasicStatistics> = roster.roster.iter().filter_map(|player: &Player| {
        println!("{}", &player.person.fullName);
        if &player.position.abbreviation == PITCHER {
            return None;
        }
        Some(get(format!(hitting_stats_url!(), player.person.id)).unwrap().json().unwrap())
    }).collect();
    hitter_stats.sort_by(|stats0: &BasicStatistics, stats1: &BasicStatistics|
        stats0.stats.0.splits[0].stat.gamesPlayed.cmp(&stats1.stats.0.splits[0].stat.gamesPlayed)
    );
    println!("{}", hitter_stats.len());
    hitter_stats
}

fn get_team_pitching_stats(roster: &Roster) {
    let mut pitcher_stats =
}

pub(crate) fn display_team_stats(query: &Vec<String>) {
    const ID_LEN: usize = 3;
    let team = &query[1];
    println!("{}", team);

    let team_id = get_id("database/team_ids.txt", team, ID_LEN).unwrap();
    if team_id.is_positive() {
        let roster: Roster = get(format!(roster_url!(), team_id)).unwrap().json().unwrap();
        get_team_hitting_stats(&roster);
    }
    else {
        println!("Invalid Team!")
    }
}
