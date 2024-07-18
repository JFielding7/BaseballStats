use std::{env, io};
use std::fs::File;
use std::io::{Read, Seek, Result};

mod hitting_stats;
mod database_generator;
mod teams;

fn main() {
    hitting_stats::display_hitting_stats(&"ronald-acuna-jr".to_string());
}