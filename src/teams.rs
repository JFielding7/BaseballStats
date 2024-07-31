use std::cmp::Ordering;
use std::{env};
use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::stats::{get_entry, Stat};
use crate::hitting_stats::{get_basic_season_hitting_stats, get_basic_hitting_row, BasicHittingStats, basic_hitting_header, Batter, basic_hitting_row};
use crate::pitching_stats::{get_season_pitching_stats, get_pitching_row, PitchingStats, pitching_header, Pitcher, pitching_row};
use crate::query::{empty, get_query_param, QueryError};

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
}

#[derive(Deserialize)]
struct Position {
    abbreviation: String
}

#[derive(Deserialize)]
struct TeamStats {
    stats: (Stat<Batter>, Stat<Pitcher>)
}

const PITCHER: &str = "P";

macro_rules! roster_url {
    () => { "https://statsapi.mlb.com/api/v1/teams/{}/roster?rosterType=fullSeason" };
}

macro_rules! stats_url {
    ($team_id:expr) => {
        format!("https://statsapi.mlb.com/api/v1/teams/{}/stats?group=pitching,hitting&stats=season", $team_id)
    };
}

macro_rules! database_file {
    ($file:expr) => { &format!("{}/database/{}", env!("CARGO_MANIFEST_DIR"), $file) };
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

fn get_team_roster(team_id: i32) -> reqwest::Result<(Vec<Player>, Vec<Player>)> {
    let roster: Roster = get(format!(roster_url!(), team_id))?.json()?;
    Ok(roster.roster.into_iter().partition(|player| player.position.abbreviation == PITCHER))
}

fn display_team_season_stats(team_name: String, team_id: i32, display_hitting: bool, display_pitching: bool) -> reqwest::Result<()> {
    let (pitchers, hitters) = get_team_roster(team_id)?;
    let team_stats: TeamStats = get(stats_url!(team_id))?.json()?;

    if display_hitting {
        let mut stat_table: Table = stat_table!(BasicHittingStats, basic_hitting_header, hitters,
            get_basic_season_hitting_stats, get_basic_hitting_row, hitter_comparator);
        let split = &team_stats.stats.0.splits[0];
        stat_table.add_row(basic_hitting_row!("Team", &split.stat));
        println!("\n{}Hitting Stats\n\n{}", team_name, stat_table.render());
    }

    if display_pitching {
        let mut stat_table: Table = stat_table!(PitchingStats, pitching_header, pitchers,
            get_season_pitching_stats, get_pitching_row, pitcher_comparator);
        stat_table.add_row(pitching_row!("Team", &team_stats.stats.1.splits[0].stat));
        println!("\n{}Pitching Stats\n\n{}", team_name, stat_table.render());
    }
    Ok(())
}

pub(crate) fn get_team(abbreviation: &String) -> Result<(Vec<String>, i32), QueryError> {
    const ID_LEN: usize = 3;

    let entry = get_entry(database_file!("team_ids.txt"), abbreviation, ID_LEN)?;
    let team_id = entry[entry.len() - 1].parse::<i32>().unwrap();
    Ok((entry, team_id))
}

pub(crate) fn display_team_stats(query: &Vec<String>) -> Result<(), QueryError> {
    const TEAM_INDEX: usize = 2;
    const STAT_INDEX: usize = 3;
    const MIN_LENGTH: usize = 3;

    if query.len() < MIN_LENGTH {
        return Err(QueryError::QueryTooShort("No Team Provided".to_string()));
    }

    let team = &query[TEAM_INDEX];
    let (entry, team_id) = get_team(team)?;
    let mut name: String = "".to_string();
    for i in 1..(entry.len() - 1) {
        name.push_str(&format!("{} ", entry[i].clone()));
    }

    let (display_hitting, display_pitching) =
        match get_query_param!(query, STAT_INDEX, empty!()).as_str() {
            "h" | "hitting" => (true, false),
            "p" | "pitching" => (false, true),
            _ => (true, true)
        };

    display_team_season_stats(name, team_id, display_hitting, display_pitching)?;
    Ok(())
}
