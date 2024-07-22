use std::env;
use chrono::format;
use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::stats::get_entry;
use crate::{hitting_stats, pitching_stats};
use crate::hitting_stats::BasicHittingStats;
use crate::pitching_stats::PitchingStats;

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

macro_rules! database_file {
    ($file:expr) => { &format!("{}/database/{}", env::current_dir().unwrap().display(), $file) };
}

fn get_team_stats(team_id: i32) -> (Vec<Player>, Vec<Player>) {
    let roster: Roster = get(format!(roster_url!(), team_id)).unwrap().json().unwrap();
    roster.roster.into_iter().partition(|player| player.position.abbreviation == PITCHER)
}

pub(crate) fn print_team_stats(name: String, team_id: i32, display_hitting: bool, display_pitching: bool) {
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
        println!("\n{} Hitting Stats\n\n{}", name, stat_table.render());
    }

    if display_pitching {
        let mut pitcher_stats: Vec<PitchingStats> = pitchers
            .iter()
            .filter_map(|pitcher| {
                let stat_result = pitching_stats::get_pitching_stats(pitcher.person.id, "season");
                match stat_result {
                    Ok(stat) => Some(stat),
                    Err(e) => None
                }
            })
            .collect();
        pitcher_stats.sort_by(|player0, player1| player1.stats[0].splits[0].stat.inningsPitched.cmp(&player0.stats[0].splits[0].stat.inningsPitched));

        let mut stat_table = Table::new();
        stat_table.add_row(hitting_stats::basic_hitting_header!("Player"));
        pitcher_stats.iter().for_each(|pitcher| stat_table.add_row(hitting_stats::get_basic_hitting_row(pitcher)));
        println!("\n{} Hitting Stats\n\n{}", name, stat_table.render());
    }
}

pub(crate) fn display_team_stats(query: &Vec<String>) {
    const ID_LEN: usize = 3;
    let team = &query[1];

    let entry = get_entry(database_file!("team_ids.txt"), team, ID_LEN).unwrap();

    let team_id = entry[entry.len() - 1].parse::<i32>().unwrap();
    if team_id.is_positive() {
        let mut name: String = "".to_string();
        for i in 1..(entry.len() - 1) {
            name.push_str(&format!("{} ", entry[i].clone()));
        }
        print_team_stats(name, team_id, true, false);
    }
    else {
        println!("Invalid Team!")
    }
}
