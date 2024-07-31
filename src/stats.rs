use serde::Deserialize;

use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::hitting_stats::display_hitting_stats;
use crate::pitching_stats::display_pitching_stats;
use crate::query::{get_query_param, QueryError};
use crate::query::QueryError::EntryError;

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
    () => { &format!("{}/database/player_ids.txt", env!("CARGO_MANIFEST_DIR")) };
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

pub(crate) fn get_entry(file: &String, key: &String, id_len: usize) -> Result<Vec<String>, QueryError> {
    let bytes = key.as_bytes();
    let mut player_file = File::open(file)?;
    let file_len = player_file.metadata()?.len();
    let line_len: u64 = get_line_length(&player_file);
    if key.len() > (line_len as usize) - id_len - 2 {
        return Err(EntryError(key.to_string()));
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
    Err(EntryError(key.to_string()))
}

pub(crate) fn stats_query(query: &Vec<String>) -> Result<(), QueryError> {
    const PLAYER_INDEX: usize = 2;
    const SEASON_TYPE_INDEX: usize = 3;
    const MIN_LENGTH: usize = 3;
    const ID_LEN: usize = 6;
    const IS_PITCHER_INDEX: usize = 1;
    const ID_INDEX: usize = 2;

    if query.len() < MIN_LENGTH  {
        return Err(QueryError::QueryTooShort("No Player Provided".to_string()));
    }

    let default_season_type: &String = &"s".to_string();
    let season_type: &str = match get_query_param!(query, SEASON_TYPE_INDEX, default_season_type).as_str() {
        "c" => "career",
        "y" => "yearByYear",
        _ => "season"
    };

    let player = &query[PLAYER_INDEX];
    let entry= get_entry(database_file!(), player, ID_LEN)?;

    let id = entry[ID_INDEX].parse::<i32>().unwrap();
    if entry[IS_PITCHER_INDEX].as_bytes()[0] != b'0' {
        display_pitching_stats(id, season_type)?;
    }
    else {
        display_hitting_stats(id, season_type)?;
    }
    Ok(())
}
