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

use crate::game::{display_team_past_games, display_game_stats, display_games_today, display_schedule};

fn main() {
    // display_games_today();
    // display_team_past_games(143, 8);
    // display_schedule(143, 4);
    display_game_stats(745315);
    // display_standings().expect("Fail");
    // let query = env::args().collect();
    // teams::display_team_stats(&query);
    // stats::display_stats(&query);
    // pitching_stats::display_pitching_stats(&query);
    // hitting_stats::display_hitting_stats(&query);
}