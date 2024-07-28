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

use crate::game::{display_team_past_games, display_game_stats, display_games_today, display_schedule, games_query};
use crate::leaders::display_leaders;
use crate::stats::Stat;

fn main() {
    let query: Vec<String> = env::args().collect();
    match query.get(1).unwrap_or(&"".to_string()).as_str() {
        "g" => {
            games_query(&query);
        }
        _ => {

        }
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