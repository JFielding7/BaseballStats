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

use figlet_rs::FIGfont;
use crate::game::{display_game_stats, display_games_today};
// fn main() {
//     let small_font = FIGfont::standard().unwrap();
//     let figure0 = small_font.convert("PHI 11 - 0 MIN").unwrap();
//     let figure1 = small_font.convert("MIN 0").unwrap();
//
//     println!("{}", figure0);
//     println!("{}", figure1);
// }

fn main() {
    // display_games_today();
    display_game_stats(746199);
    // display_standings().expect("Fail");
    // let query = env::args().collect();
    // teams::display_team_stats(&query);
    // stats::display_stats(&query);
    // pitching_stats::display_pitching_stats(&query);
    // hitting_stats::display_hitting_stats(&query);
}