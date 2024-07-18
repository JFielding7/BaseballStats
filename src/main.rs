use std::env;

mod hitting_stats;
mod database_generator;
mod teams;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("hello {:?}", args);
    // hitting_stats::display_hitting_stats();
}