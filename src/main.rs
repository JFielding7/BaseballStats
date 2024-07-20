use std::{env};
use std::error::Error;
use crate::database_generator::update_teams;

mod hitting_stats;
mod database_generator;
mod teams;
mod data_id;
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
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Use `iter` to create an iterator that borrows each element
    let even_numbers: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

    // Print the filtered vector of references
    println!("{:?}", even_numbers);

    // The original vector is still accessible
    println!("{:?}", numbers);
    // let v = vec![0, 1, 2, 3];
    // let w: Vec<i32> = v.into_iter().filter(|num| (*num & 1) == 0).collect();
    // println!("{:?}", w);
    // let query = env::args().collect();
    // hitting_stats::display_hitting_stats(&query);
    // update_teams().expect("Fail");
}