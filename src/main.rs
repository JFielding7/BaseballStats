use std::{env};
use std::error::Error;
use crate::database_generator::update_teams;

mod hitting_stats;
mod database_generator;
mod teams;
mod pitching_stats;
mod stats;
// #[derive(Debug)]
// enum PlayerError {
//     NoPlayer
// }
//
// impl Error for PlayerError {}
//
// impl Display for PlayerError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let m = "hello";
//         write!(f, "Incorrect {m}")
//     }
// }
//
// fn f(a: i32) -> Result<i32, PlayerError> {
//     if a == 0 {
//         return Err(PlayerError::NoPlayer);
//     }
//     Ok(a)
// }

fn main() {
    // database_generator::update_players(false).unwrap()
    let query = env::args().collect();
    teams::display_team_stats(&query);
    // stats::display_stats(&query);
    // pitching_stats::display_pitching_stats(&query);
    // hitting_stats::display_hitting_stats(&query);
    // update_teams().unwrap()
}