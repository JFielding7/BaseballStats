use std::fmt::{Display};
use serde::Deserialize;
use term_table::table_cell::TableCell;
use term_table::row::Row;
use term_table::{row, Table};
use reqwest::blocking::get;
use crate::stats::{Split, Stat};

#[derive(Deserialize)]
pub(crate) struct BasicHittingStats {
    pub(crate) stats: (Stat<Batter>,)
}

#[derive(Deserialize)]
struct FullHittingStats {
    stats: (Stat<Batter>, Stat<AdvancedBatter>)
}

#[derive(Deserialize)]
struct YearByYearStats {
    stats: (Stat<Batter>, Stat<Batter>, Stat<AdvancedBatter>, Stat<AdvancedBatter>)
}

#[derive(Deserialize)]
pub(crate) struct Batter {
    pub(crate) plateAppearances: i32,
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
    totalBases: i32,
    rbi: i32,
    leftOnBase: i32,
    sacBunts: i32,
    sacFlies: i32,
    atBatsPerHomeRun: String
}

#[derive(Deserialize)]
struct AdvancedBatter {
    pitchesPerPlateAppearance: String,
    walksPerPlateAppearance: String,
    strikeoutsPerPlateAppearance: String,
    homeRunsPerPlateAppearance: String,
    walksPerStrikeout: String,
}

macro_rules! career_years_url {
    () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats=yearByYear,career,yearByYearAdvanced,careerAdvanced&group=hitting" };
}

macro_rules! basic_season_stats_url {
    () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats=season&group=hitting" };
}

macro_rules! advanced_group_url {
    () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats={},{}Advanced&group=hitting" };
}

macro_rules! basic_hitting_header {
    ($col0:expr) => {
        row!($col0, "G", "PA", "AB", "R", "H", "2B", "3B", "HR", "RBI", "BA", "OBP", "SLG", "OPS", "SO", "BB", "HBP")
    };
}
pub(crate) use basic_hitting_header;

macro_rules! advanced_hitting_header {
    ($col0:expr) => {
        row!($col0, "TB", "SH", "SF", "IBB", "SB", "CS", "SBP", "GDP", "LOB", "P/PA", "BB/PA", "SO/PA", "BB/SO", "HR/PA", "AB/HR")
    };
}

macro_rules! basic_hitting_row {
    ($col0:expr, $split:expr, $stat_group:expr) => {
        row!(
            $col0, $stat_group.gamesPlayed, $stat_group.plateAppearances,
            $stat_group.atBats, $stat_group.runs, $stat_group.hits, $stat_group.doubles,
            $stat_group.triples, $stat_group.homeRuns, $stat_group.rbi, &$stat_group.avg,
            &$stat_group.obp, &$stat_group.slg, &$stat_group.ops, $stat_group.strikeOuts,
            $stat_group.baseOnBalls, $stat_group.hitByPitch
        )
    };
}

pub(crate) fn get_basic_hitting_row(stats: &BasicHittingStats) -> Row {
    let split = &stats.stats.0.splits[0];
    let stat_group = &split.stat;
    basic_hitting_row!(&stats.stats.0.splits[0].player.fullName, split, stat_group)
}

fn advanced_hitting_row(advanced_split: &Split<AdvancedBatter>, stat_group: &Batter, advanced_stat_group: &AdvancedBatter) -> Row {
    row!(
        &advanced_split.season, stat_group.totalBases, stat_group.sacBunts,
        stat_group.sacFlies, stat_group.intentionalWalks, stat_group.stolenBases,
        stat_group.caughtStealing, &stat_group.stolenBasePercentage,
        stat_group.groundIntoDoublePlay, stat_group.leftOnBase,
        &advanced_stat_group.pitchesPerPlateAppearance, &advanced_stat_group.walksPerPlateAppearance,
        &advanced_stat_group.strikeoutsPerPlateAppearance, &advanced_stat_group.walksPerStrikeout,
        &advanced_stat_group.homeRunsPerPlateAppearance, &stat_group.atBatsPerHomeRun
    )
}

pub(crate) fn get_basic_season_hitting_stats(player_id: i32) -> reqwest::Result<BasicHittingStats> {
    get(format!(basic_season_stats_url!(), player_id)).unwrap().json()
}

fn get_hitting_stats(player_id: i32, season_type: &str) -> (Vec<Stat<Batter>>, Vec<Stat<AdvancedBatter>>) {
    if season_type == "yearByYear" {
        let url = format!(career_years_url!(), player_id);
        let stats: YearByYearStats = get(url).unwrap().json().unwrap();
        return (vec![stats.stats.0, stats.stats.1], vec![stats.stats.2, stats.stats.3]);
    }
    let url = format!(advanced_group_url!(), player_id, season_type, season_type);
    let stats: FullHittingStats = get(url).unwrap().json().unwrap();
    (vec![stats.stats.0], vec![stats.stats.1])
}

pub(crate) fn display_hitting_stats(player_id: i32, season_type: &str) {
    let stats: (Vec<Stat<Batter>>, Vec<Stat<AdvancedBatter>>) = get_hitting_stats(player_id, season_type);

    let mut table0 = Table::new();
    table0.add_row(basic_hitting_header!("Year"));

    let mut table1 = Table::new();
    table1.add_row(advanced_hitting_header!("Year"));

    let reg_stats = &stats.0;
    let advanced_stats = &stats.1;
    for i in 0..reg_stats.len() {
        let reg_splits = &reg_stats[i].splits;
        let advanced_splits = &advanced_stats[i].splits;

        for j in 0..reg_splits.len() {
            let split = &reg_splits[j];
            let stat_group = &split.stat;
            table0.add_row(basic_hitting_row!(&split.season, split, stat_group));

            let advanced_split = &advanced_splits[j];
            let advanced_stat_group = &advanced_split.stat;
            table1.add_row(advanced_hitting_row(advanced_split, stat_group, advanced_stat_group));
        }
    }

    println!("\nPlayer: {}\n\nStandard Batting:\n{}", &stats.0[0].splits[0].player.fullName, table0.render());
    println!("Advanced Batting:\n{}", table1.render());
}
