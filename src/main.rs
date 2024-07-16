mod batter_stats;
mod database;
mod teams;

fn main() {
    database::update_players(true);
}