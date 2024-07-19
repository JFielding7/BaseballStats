use std::{env};
use std::error::Error;

mod hitting_stats;
mod database_generator;
mod teams;

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
    let query = env::args().collect();
    hitting_stats::display_hitting_stats(query);
}