use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::stats::get_id;
use crate::hitting_stats;
use crate::hitting_stats::BasicHittingStats;

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

// macro_rules! hitting_stats_url {
//     () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats=season&group=hitting" };
// }
//
// macro_rules! pitching_stats_url {
//     () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats=season&group=pitching" };
// }

// fn get_team_hitting_stats() {
//     let hitter_stats: Vec<BasicHittingStats> = hitters.iter().map(|&hitter| get(format!(hitting_stats_url!(), hitter.person.id)).unwrap().json().unwrap());
// }
//
// fn get_team_pitching_stats() {
//     let pitcher_stats: Vec<PitchingStats> = pitchers.iter().map(|&pitcher| get(format!(pitching_stats_url!(), pitcher.person.id)).unwrap().json().unwrap());
// }

fn get_team_stats(team_id: i32) -> (Vec<Player>, Vec<Player>) {
    let roster: Roster = get(format!(roster_url!(), team_id)).unwrap().json().unwrap();
    roster.roster.into_iter().partition(|player| player.position.abbreviation == PITCHER)
}

pub(crate) fn print_team_stats(team_id: i32, display_hitting: bool, display_pitching: bool) {
    let (pitchers, hitters) = get_team_stats(team_id);

    if display_hitting {
        let mut hitter_stats: Vec<BasicHittingStats> = hitters
            .iter()
            .filter_map(|hitter| {
                let stat_result = hitting_stats::get_basic_season_hitting_stats(hitter.person.id);
                match stat_result {
                    Ok(stat) => Some(stat),
                    Err(e) => None
                }
            })
            .collect();
        hitter_stats.sort_by(|player0, player1| player1.stats.0.splits[0].stat.plateAppearances.cmp(&player0.stats.0.splits[0].stat.plateAppearances));

        let mut stat_table = Table::new();
        stat_table.add_row(hitting_stats::basic_hitting_header!("Player"));
        hitter_stats.iter().for_each(|hitter| stat_table.add_row(hitting_stats::get_basic_hitting_row(hitter)));
        println!("{}", stat_table.render());
    }

    if display_pitching {
        // TODO: print pitching stats
    }
}

pub(crate) fn display_team_stats(query: &Vec<String>) {
    const ID_LEN: usize = 3;
    let team = &query[1];
    println!("{}", team);

    let (team_id, _) = get_id("database/team_ids.txt", team, ID_LEN).unwrap();
    if team_id.is_positive() {
        print_team_stats(team_id, true, false);
    }
    else {
        println!("Invalid Team!")
    }
}
