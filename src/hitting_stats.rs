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
    #[serde(default)]
    pub(crate) gamesPlayed: i32,
    pub(crate) runs: i32,
    pub(crate) doubles: i32,
    pub(crate) triples: i32,
    pub(crate) homeRuns: i32,
    pub(crate) strikeOuts: i32,
    pub(crate) baseOnBalls: i32,
    pub(crate) intentionalWalks: i32,
    pub(crate) hits: i32,
    pub(crate) hitByPitch: i32,
    #[serde(default)]
    pub(crate) avg: String,
    pub(crate) atBats: i32,
    #[serde(default)]
    pub(crate) obp: String,
    #[serde(default)]
    pub(crate) slg: String,
    #[serde(default)]
    pub(crate) ops: String,
    pub(crate) caughtStealing: i32,
    pub(crate) stolenBases: i32,
    pub(crate) stolenBasePercentage: String,
    pub(crate) groundIntoDoublePlay: i32,
    pub(crate) totalBases: i32,
    pub(crate) rbi: i32,
    pub(crate) leftOnBase: i32,
    pub(crate) sacBunts: i32,
    pub(crate) sacFlies: i32,
    pub(crate) atBatsPerHomeRun: String
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
    ($col0:expr, $stat_group:expr) => {
        row!(
            $col0, $stat_group.gamesPlayed, $stat_group.plateAppearances,
            $stat_group.atBats, $stat_group.runs, $stat_group.hits, $stat_group.doubles,
            $stat_group.triples, $stat_group.homeRuns, $stat_group.rbi, &$stat_group.avg,
            &$stat_group.obp, &$stat_group.slg, &$stat_group.ops, $stat_group.strikeOuts,
            $stat_group.baseOnBalls, $stat_group.hitByPitch
        )
    };
}
pub(crate) use basic_hitting_row;

pub(crate) fn get_basic_hitting_row(stats: &BasicHittingStats) -> Row {
    let split = &stats.stats.0.splits[0];
    basic_hitting_row!(&split.player.fullName, &split.stat)
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
    let stats: BasicHittingStats = get(format!(basic_season_stats_url!(), player_id))?.json()?;
    Ok(stats)
}

fn get_hitting_stats(player_id: i32, season_type: &str) -> reqwest::Result<(Vec<Stat<Batter>>, Vec<Stat<AdvancedBatter>>)> {
    if season_type == "yearByYear" {
        let url = format!(career_years_url!(), player_id);
        let stats: YearByYearStats = get(url)?.json()?;
        return Ok((vec![stats.stats.0, stats.stats.1], vec![stats.stats.2, stats.stats.3]));
    }
    let url = format!(advanced_group_url!(), player_id, season_type, season_type);
    let stats: FullHittingStats = get(url)?.json()?;
    Ok((vec![stats.stats.0], vec![stats.stats.1]))
}

pub(crate) fn display_hitting_stats(player_id: i32, season_type: &str) -> reqwest::Result<()> {
    let stats: (Vec<Stat<Batter>>, Vec<Stat<AdvancedBatter>>) = get_hitting_stats(player_id, season_type)?;

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
            table0.add_row(basic_hitting_row!(&split.season, stat_group));

            let advanced_split = &advanced_splits[j];
            let advanced_stat_group = &advanced_split.stat;
            table1.add_row(advanced_hitting_row(advanced_split, stat_group, advanced_stat_group));
        }
    }

    println!("\nPlayer: {}\n\nStandard Batting:\n{}", &stats.0[0].splits[0].player.fullName, table0.render());
    println!("Advanced Batting:\n{}", table1.render());
    Ok(())
}
