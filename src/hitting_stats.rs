use std::fs::File;
use std::io::{BufReader, Read, Result, Seek, SeekFrom};
use std::process::id;
use serde::Deserialize;
use term_table::table_cell::TableCell;
use term_table::row::Row;
use term_table::{Table};

#[derive(Deserialize)]
pub(crate) struct Statistics {
    stats: (Stat, AdvancedStat)
}

#[derive(Deserialize)]
struct Stat {
    splits: (Split,)
}

#[derive(Deserialize)]
struct Split {
    stat: BatterStats
}

#[derive(Deserialize)]
struct AdvancedStat {
    splits: (AdvancedSplit, )
}

#[derive(Deserialize)]
struct AdvancedSplit {
    stat: BatterAdvancedStats
}

#[derive(Deserialize)]
struct BatterStats {
    gamesPlayed: i32,
    runs: i32,
    doubles: i32,
    triples: i32,
    homeRuns: i32,
    strikeOuts: i32,
    baseOnBalls: i32,
    intentionalWalks: i32,
    hits: i32,
    hitByPitch: i32,
    avg: String,
    atBats: i32,
    obp: String,
    slg: String,
    ops: String,
    caughtStealing: i32,
    stolenBases: i32,
    stolenBasePercentage: String,
    groundIntoDoublePlay: i32,
    plateAppearances: i32,
    totalBases: i32,
    rbi: i32,
    leftOnBase: i32,
    sacBunts: i32,
    sacFlies: i32,
    atBatsPerHomeRun: String
}

#[derive(Deserialize)]
struct BatterAdvancedStats {
    pitchesPerPlateAppearance: String,
    walksPerPlateAppearance: String,
    strikeoutsPerPlateAppearance: String,
    homeRunsPerPlateAppearance: String,
    walksPerStrikeout: String,
    iso: String
}

macro_rules! table {
    ($cols:expr, $($cell:expr),*) => {
        {
            let mut table = Table::new();
            let mut row = Vec::new();
            let mut i = 0;
            $(
                row.push(TableCell::new($cell));
                i += 1;
                if i == $cols {
                    table.add_row(Row::new(row));
                    row = Vec::new();
                    i = 0;
                }
            )*
            table
        }
    };
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

fn get_player_id(player: &String) -> Result<(i32)> {
    const ID_LENGTH: usize = 6;

    let bytes = player.as_bytes();
    let mut player_file = File::open("database/player_ids.txt")?;
    let file_len = player_file.metadata().unwrap().len();
    let line_len: u64 = get_line_length(&player_file);
    if player.len() > (line_len as usize) - ID_LENGTH - 2 {
        return Ok(-1);
    }

    let mut start = 0;
    let mut end = file_len;
    let mut buffer: Box<[u8]> = vec![0; line_len as usize].into_boxed_slice();
    while start < end {
        let mid = (start + end >> 1) / line_len * line_len;
        player_file.seek(SeekFrom::Start(mid))?;
        player_file.read_exact(&mut buffer)?;

        println!("{} {} {}: {}", start, mid, end, String::from_utf8(buffer.to_vec()).unwrap());

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
            player_file.seek(SeekFrom::Start(mid + line_len - (ID_LENGTH as u64) - 1))?;
            let mut id_buffer: [u8; ID_LENGTH] = [0; ID_LENGTH];
            player_file.read_exact(&mut id_buffer)?;
            return Ok(String::from_utf8_lossy(&id_buffer).parse::<i32>().unwrap());
        }
        else if cmp > 0 {
            start = mid + line_len;
        }
        else {
            end = mid;
        }
    }
    Ok(-1)
}

fn get_hitting_stats(player_id: i32) -> Statistics {
    let url = format!("https://statsapi.mlb.com/api/v1/people/{}/stats?stats=career,careerAdvanced&group=hitting", player_id);
    let response: Statistics = reqwest::blocking::get(url).unwrap().json().unwrap();
    response
}

pub(crate) fn display_hitting_stats(player: &String) {
    println!("{:?}", get_player_id(player));
    // let table = table!(3, "Name", "Age", "Occupation", "John Doe", "30", "Software Developer");
    // println!("{}", table.render());
}