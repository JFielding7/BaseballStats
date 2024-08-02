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
use crate::database::update_database;
use crate::standings::display_standings;
use crate::game::{games_query, season_games_query};
use crate::leaders::display_leaders;
use crate::league_averages::display_league_averages;
use crate::query::{empty, get_query_param};
use crate::stats::{stats_query};
use crate::teams::display_team_stats;

fn main() {
    const QUERY_TYPE_INDEX: usize = 1;

    let query: Vec<String> = env::args().collect();
    let res= match get_query_param!(&query, QUERY_TYPE_INDEX, empty!()).as_str() {
        "g" | "games" => games_query(&query),
        "r" | "results" => season_games_query(&query),
        "u" | "schedule" => season_games_query(&query),
        "s" | "stats" => stats_query(&query),
        "t" | "teams" => display_team_stats(&query),
        "l" | "leaders" => display_leaders(&query),
        "b" | "league-batting-stats" => display_league_averages(&query, true),
        "p" | "league-pitching-stats" => display_league_averages(&query, false),
        "update" => update_database(&query),
        _ => display_standings()
    };

    match res {
        Ok(_) => {},
        Err(e) => eprintln!("{e}")
    }
}