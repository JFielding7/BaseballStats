use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Result, Seek, SeekFrom};
use std::process::id;
use std::str::ParseBoolError;
use chrono::format::Item::Error;
use serde::Deserialize;
use term_table::table_cell::TableCell;
use term_table::row::Row;
use term_table::{Table};

#[derive(Deserialize, Debug)]
pub(crate) struct Statistics {
    stats: (Stat, AdvancedStat)
}

#[derive(Deserialize, Debug)]
struct Stat {
    splits: Vec<Split>
}

#[derive(Deserialize, Debug)]
struct Split {
    #[serde(default = "default_season")]
    season: String,
    stat: BatterStats,
    player: Player
}

#[derive(Deserialize, Debug)]
struct Player {
    fullName: String
}

#[derive(Deserialize, Debug)]
struct AdvancedStat {
    splits: Vec<AdvancedSplit>
}

#[derive(Deserialize, Debug)]
struct AdvancedSplit {
    #[serde(default = "default_season")]
    season: String,
    stat: AdvancedBatterStats
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
struct AdvancedBatterStats {
    pitchesPerPlateAppearance: String,
    walksPerPlateAppearance: String,
    strikeoutsPerPlateAppearance: String,
    homeRunsPerPlateAppearance: String,
    walksPerStrikeout: String,
}

fn default_season() -> String {
    "ALL".to_string()
}

// macro_rules! table {
//     ($cols:expr, $($cell:expr),*) => {
//         {
//             let mut table = Table::new();
//             let mut row = Vec::new();
//             let mut i = 0;
//             $(
//                 row.push(TableCell::new($cell));
//                 i += 1;
//                 if i == $cols {
//                     table.add_row(Row::new(row));
//                     row = Vec::new();
//                     i = 0;
//                 }
//             )*
//             table
//         }
//     };
// }

macro_rules! row {
    ($($cell:expr),*) => {
        {
            let mut row = Vec::new();
            $(row.push(TableCell::new($cell));)*
            Row::new(row)
        }
    };
}

fn display_stat_table(stats: &Statistics) -> Table {
    let mut table = Table::new();
    let header = row!("Year", "G", "PA", "AB", "R", "H", "2B", "3B", "HR", "RBI", "BA", "OBP", "SLG", "OPS", "SO", "BB", "HBP");
    table.add_row(header);

    for split in &stats.stats.0.splits {
        let stat_group = &split.stat;
        table.add_row(row!(&split.season, stat_group.gamesPlayed, stat_group.plateAppearances,
            stat_group.atBats, stat_group.runs, stat_group.hits, stat_group.doubles,
            stat_group.triples, stat_group.homeRuns, stat_group.rbi, &stat_group.avg,
            &stat_group.obp, &stat_group.slg, &stat_group.ops, stat_group.strikeOuts,
            stat_group.baseOnBalls, stat_group.hitByPitch));
    }
    table
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

fn get_player_id(player: &String) -> Result<i32> {
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

fn get_hitting_stats(player_id: i32, season_type: &str) -> Statistics {
    let url = format!("https://statsapi.mlb.com/api/v1/people/{}/stats?stats={},{}Advanced&group=hitting", player_id, season_type, season_type);
    reqwest::blocking::get(url).unwrap().json().unwrap()
}

pub(crate) fn display_hitting_stats(query: Vec<String>) {
    if query.len() == 1 {
        return;
    }
    let season_type: &str;
    if query.len() == 2 {
        season_type = "season";
    }
    else {
        match query[2].to_ascii_lowercase().chars().next().unwrap() {
            'c' => season_type = "career",
            'y' => season_type = "yearByYear",
            _ => season_type = "season"
        }
    }
    let stats = get_hitting_stats(get_player_id(&query[1]).unwrap(), season_type);
    println!("\nPlayer: {}\n", &stats.stats.0.splits[0].player.fullName);
    let table = display_stat_table(&stats);
    println!("{}", table.render());
}