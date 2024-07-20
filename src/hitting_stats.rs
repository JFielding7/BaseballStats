use std::fmt::{Display};
use serde::Deserialize;
use term_table::table_cell::TableCell;
use term_table::row::Row;
use term_table::{Table};
use crate::data_id::get_id;

#[derive(Deserialize)]
pub(crate) struct BasicStatistics {
    stats: Stat
}

#[derive(Deserialize)]
struct FullStatistics {
    stats: (Stat, AdvancedStat)
}

#[derive(Deserialize)]
struct YearByYearStats {
    stats: (Stat, Stat, AdvancedStat, AdvancedStat)
}

#[derive(Deserialize)]
struct Stat {
    splits: Vec<Split>
}

#[derive(Deserialize)]
struct Split {
    #[serde(default = "default_season")]
    season: String,
    stat: BatterStats,
    player: Player
}

#[derive(Deserialize)]
struct Player {
    fullName: String
}

#[derive(Deserialize)]
struct AdvancedStat {
    splits: Vec<AdvancedSplit>
}

#[derive(Deserialize)]
struct AdvancedSplit {
    #[serde(default = "default_season")]
    season: String,
    stat: AdvancedBatterStats
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
struct AdvancedBatterStats {
    pitchesPerPlateAppearance: String,
    walksPerPlateAppearance: String,
    strikeoutsPerPlateAppearance: String,
    homeRunsPerPlateAppearance: String,
    walksPerStrikeout: String,
}

fn default_season() -> String {
    "Career".to_string()
}

macro_rules! row {
    ($($cell:expr),*) => {
        {
            let mut row = Vec::new();
            $(row.push(TableCell::new($cell));)*
            Row::new(row)
        }
    };
}

fn get_hitting_stats(player_id: i32, season_type: &str) -> (Vec<Stat>, Vec<AdvancedStat>) {
    if season_type == "yearByYear" {
        let url = format!("https://statsapi.mlb.com/api/v1/people/{}/stats?stats=yearByYear,career,yearByYearAdvanced,careerAdvanced&group=hitting", player_id);
        let stats: YearByYearStats = reqwest::blocking::get(url).unwrap().json().unwrap();
        return (vec![stats.stats.0, stats.stats.1], vec![stats.stats.2, stats.stats.3]);
    }
    let url = format!("https://statsapi.mlb.com/api/v1/people/{}/stats?stats={},{}Advanced&group=hitting", player_id, season_type, season_type);
    let stats: FullStatistics = reqwest::blocking::get(url).unwrap().json().unwrap();
    (vec![stats.stats.0], vec![stats.stats.1])
}

fn display_stats(stats: (Vec<Stat>, Vec<AdvancedStat>)) {
    let mut table0 = Table::new();
    table0.add_row(row!("Year", "G", "PA", "AB", "R", "H", "2B", "3B", "HR", "RBI", "BA", "OBP", "SLG", "OPS", "SO", "BB", "HBP"));

    let mut table1 = Table::new();
    table1.add_row(row!("Year", "TB", "SB", "SF", "IBB", "SB", "CS", "SBP", "GDP", "LOB", "P/PA", "BB/PA", "SO/PA", "BB/SO", "HR/PA", "AB/HR"));

    let reg_stats = &stats.0;
    let advanced_stats = &stats.1;
    for i in 0..reg_stats.len() {
        let reg_splits = &reg_stats[i].splits;
        let advanced_splits = &advanced_stats[i].splits;

        for j in 0..reg_splits.len() {
            let split = &reg_splits[j];
            let stat_group = &split.stat;
            table0.add_row(row!(&split.season, stat_group.gamesPlayed, stat_group.plateAppearances,
                stat_group.atBats, stat_group.runs, stat_group.hits, stat_group.doubles,
                stat_group.triples, stat_group.homeRuns, stat_group.rbi, &stat_group.avg,
                &stat_group.obp, &stat_group.slg, &stat_group.ops, stat_group.strikeOuts,
                stat_group.baseOnBalls, stat_group.hitByPitch)
            );

            let advanced_split = &advanced_splits[j];
            let advanced_stat_group = &advanced_split.stat;
            table1.add_row(row!(&advanced_split.season, stat_group.totalBases, stat_group.sacBunts,
                stat_group.sacFlies, stat_group.intentionalWalks, stat_group.stolenBases,
                stat_group.caughtStealing, &stat_group.stolenBasePercentage,
                stat_group.groundIntoDoublePlay, stat_group.leftOnBase,
                &advanced_stat_group.pitchesPerPlateAppearance, &advanced_stat_group.walksPerPlateAppearance,
                &advanced_stat_group.strikeoutsPerPlateAppearance, &advanced_stat_group.walksPerStrikeout,
                &advanced_stat_group.homeRunsPerPlateAppearance, &stat_group.atBatsPerHomeRun)
            );
        }
    }

    println!("\nPlayer: {}\n\nStandard Batting:\n{}", &stats.0[0].splits[0].player.fullName, table0.render());
    println!("Advanced Batting:\n{}", table1.render());
}

pub(crate) fn display_hitting_stats(query: &Vec<String>) {
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

    const ID_LEN: usize = 6;
    let id = get_id("database/player_ids.txt", &query[1], ID_LEN).unwrap();
    if id.is_positive() {
        display_stats(get_hitting_stats(id, season_type));
    }
    else {
        println!("Invalid player!");
    }
}