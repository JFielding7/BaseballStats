use std::{env};
use std::error::Error;
use crate::database::update_teams;
use crate::standings::display_standings;

mod hitting_stats;
mod database;
mod teams;
mod pitching_stats;
mod stats;
mod standings;
mod game;
mod leaders;
mod query;

use crate::game::{display_game_stats, display_games_today, games_query, season_games_query};
use crate::leaders::display_leaders;
use crate::query::{empty, get_query_param};
use crate::stats::{stats_query};
use crate::teams::display_team_stats;

fn main() {
    const QUERY_TYPE_INDEX: usize = 1;

    let query: Vec<String> = env::args().collect();
    let res= match get_query_param!(&query, QUERY_TYPE_INDEX, empty!()).as_str() {
        "g" => games_query(&query),
        "r" => season_games_query(&query),
        "u" => season_games_query(&query),
        "p" => stats_query(&query),
        "t" => display_team_stats(&query),
        "l" => display_leaders(&query),
        _ => display_standings()
    };

    match res {
        Ok(_) => {},
        Err(e) => eprintln!("{e}")
    }

    // display_leaders(&query);
    // display_games_today();
    // display_team_past_games(143, 8);
    // display_schedule(143, 4);
    // display_game_stats(744908);
    // display_standings().expect("Fail");
    // teams::display_team_stats(&query);
    // stats::display_stats(&query);
    // pitching_stats::display_pitching_stats(&query);
    // hitting_stats::display_hitting_stats(&query);
}