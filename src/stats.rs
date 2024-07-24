use serde::Deserialize;

use std::env;
use std::fmt::{Display};
use std::fs::File;
use std::io::{BufReader, Read, Result, Seek, SeekFrom};
use crate::hitting_stats::display_hitting_stats;
use crate::pitching_stats::display_pitching_stats;

#[derive(Deserialize)]
pub(crate) struct Stat<T> {
    pub(crate) splits: Vec<Split<T>>
}

#[derive(Deserialize)]
pub(crate) struct Split<T> {
    #[serde(default = "default_season")]
    pub(crate) season: String,
    #[serde(default = "no_name")]
    pub(crate) player: Player,
    pub(crate) stat: T
}

#[derive(Deserialize)]
pub(crate) struct Player {
    pub(crate) fullName: String
}

fn default_season() -> String {
    "Career".to_string()
}

fn no_name() -> Player {
    Player { fullName: "".to_string() }
}

macro_rules! database_file {
    () => { &format!("{}/database/player_ids.txt", env::current_dir().unwrap().display()) };
}

fn get_line_length(file: &File) -> u64 {
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1];

    let mut i = 1;
    while reader.read(&mut buffer).is_ok() {
        let char = buffer[0];
        if char == b'\n' {
            return i;
        }
        i += 1;
    }
    i
}

pub(crate) fn get_entry(file: &String, key: &String, id_len: usize) -> Result<Vec<String>> {
    let bytes = key.as_bytes();
    let mut player_file = File::open(file)?;
    let file_len = player_file.metadata().unwrap().len();
    let line_len: u64 = get_line_length(&player_file);
    if key.len() > (line_len as usize) - id_len - 2 {
        return Ok(vec![]);
    }

    let mut start = 0;
    let mut end = file_len;
    let mut buffer: Box<[u8]> = vec![0; line_len as usize].into_boxed_slice();
    while start < end {
        let mid = (start + end >> 1) / line_len * line_len;
        player_file.seek(SeekFrom::Start(mid))?;
        player_file.read_exact(&mut buffer)?;

        let mut cmp: i8 = 0;
        for i in 0..buffer.len() {
            if buffer[i] == b' ' {
                if i != bytes.len() {
                    cmp = 1;
                }
                break;
            }
            if i == bytes.len() {
                cmp = -1;
                break;
            }
            cmp = (bytes[i] as i8) - (buffer[i] as i8);
            if cmp != 0 {
                break;
            }
        }

        if cmp == 0 {
            return Ok(std::str::from_utf8(&buffer).unwrap().split_whitespace().map(|token| token.to_string()).collect())
        }
        else if cmp > 0 {
            start = mid + line_len;
        }
        else {
            end = mid;
        }
    }
    Ok(vec![])
}

pub(crate) fn display_stats(query: &Vec<String>) {
    const PLAYER_INDEX: usize = 2;
    const SEASON_INDEX: usize = 3;
    const MIN_LENGTH: usize = 3;
    const ID_LEN: usize = 6;

    if query.len() < MIN_LENGTH  {
        return;
    }

    let season_type: &str;
    if query.len() == SEASON_INDEX || query[SEASON_INDEX] == "s" {
        season_type = "season";
    }
    else if query[SEASON_INDEX] == "c" {
        season_type = "career";
    }
    else if query[SEASON_INDEX] == "y" {
        season_type = "yearByYear";
    }
    else {
        return;
    }

    let entry= get_entry(database_file!(), &query[PLAYER_INDEX], ID_LEN).unwrap();
    if entry.len() > 0 {
        let id = entry[2].parse::<i32>().unwrap();
        let is_pitcher = entry[1].as_bytes()[0] != b'0';
        if is_pitcher {
            display_pitching_stats(id, season_type);
        }
        else {
            display_hitting_stats(id, season_type);
        }
    }
    else {
        println!("Invalid player!");
    }
}
