use std::cmp::Ordering;
use std::env;
use chrono::format;
use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::stats::get_entry;
use crate::hitting_stats::{get_basic_season_hitting_stats, get_basic_hitting_row, BasicHittingStats, basic_hitting_header};
use crate::pitching_stats::{get_season_pitching_stats, get_pitching_row, PitchingStats, pitching_header};

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

macro_rules! stat_table {
    ($stat_type:ident, $header:ident, $players:expr, $stat_func:expr, $row_func:expr, $comparator:expr) => {{
        let mut stats: Vec<$stat_type> = $players
            .iter()
            .filter_map(|player| {
                let stat_result = $stat_func(player.person.id);
                match stat_result {
                    Ok(stat) => Some(stat),
                    Err(_e) => None
                }
            })
            .collect();
        stats.sort_by($comparator);

        let mut stat_table = Table::new();
        stat_table.add_row($header!("Player"));
        stats.iter().for_each(|player| stat_table.add_row($row_func(player)));
        stat_table
    }};
}

fn hitter_comparator(player0: &BasicHittingStats, player1: &BasicHittingStats) -> Ordering {
    player1.stats.0.splits[0].stat.plateAppearances.cmp(&player0.stats.0.splits[0].stat.plateAppearances)
}

fn pitcher_comparator(player0: &PitchingStats, player1: &PitchingStats) -> Ordering {
    player1.stats[0].splits[0].stat.inningsPitched.parse::<f32>().unwrap()
        .partial_cmp(&player0.stats[0].splits[0].stat.inningsPitched.parse::<f32>().unwrap()).unwrap()
}

fn get_team_stats(team_id: i32) -> (Vec<Player>, Vec<Player>) {
    let roster: Roster = get(format!(roster_url!(), team_id)).unwrap().json().unwrap();
    roster.roster.into_iter().partition(|player| player.position.abbreviation == PITCHER)
}

pub(crate) fn print_team_stats(team_name: String, team_id: i32, display_hitting: bool, display_pitching: bool) {
    let (pitchers, hitters) = get_team_stats(team_id);

    if display_hitting {
        let stat_table: Table = stat_table!(BasicHittingStats, basic_hitting_header, hitters,
            get_basic_season_hitting_stats, get_basic_hitting_row, hitter_comparator);
        println!("\n{}Hitting Stats\n\n{}", team_name, stat_table.render());
    }

    if display_pitching {
        let stat_table: Table = stat_table!(PitchingStats, pitching_header, pitchers,
            get_season_pitching_stats, get_pitching_row, pitcher_comparator);
        println!("\n{}Pitching Stats\n\n{}", team_name, stat_table.render());
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
        print_team_stats(name, team_id, true, true);
    }
    else {
        println!("Invalid Team!")
    }
}
