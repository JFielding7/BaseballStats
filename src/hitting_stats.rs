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

fn get_hitting_stats(player_id: i32) -> Statistics {
    let url = format!("https://statsapi.mlb.com/api/v1/people/{}/stats?stats=career,careerAdvanced&group=hitting", player_id);
    let response: Statistics = reqwest::blocking::get(url).unwrap().json().unwrap();
    response
}

pub(crate) fn display_hitting_stats(player_id: i32, season: &String) {

    let table = table!(3, "Name", "Age", "Occupation", "John Doe", "30", "Software Developer");
    println!("{}", table.render());
}