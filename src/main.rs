#![allow(non_snake_case)]
mod hitting_stats;
mod database;
mod teams;
mod pitching_stats;
mod stats;
mod standings;
mod game;
mod leaders;
mod query;
mod league_averages;

use std::{env};
use std::process::exit;
use crate::database::update_database;
use crate::standings::display_standings;
use crate::game::{games_query, season_games_query};
use crate::leaders::display_leaders;
use crate::league_averages::display_league_batting_averages;
use crate::query::{empty, get_query_param};
use crate::stats::{stats_query};
use crate::teams::display_team_stats;

fn main() {
    display_league_batting_averages();
    exit(0);
    const QUERY_TYPE_INDEX: usize = 1;

    let query: Vec<String> = env::args().collect();
    let res= match get_query_param!(&query, QUERY_TYPE_INDEX, empty!()).as_str() {
        "g" => games_query(&query),
        "r" => season_games_query(&query),
        "u" => season_games_query(&query),
        "p" => stats_query(&query),
        "t" => display_team_stats(&query),
        "l" => display_leaders(&query),
        "update" => update_database(&query),
        _ => display_standings()
    };

    match res {
        Ok(_) => {},
        Err(e) => eprintln!("{e}")
    }
}