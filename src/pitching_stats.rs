use serde::Deserialize;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::stats::{Stat};

#[derive(Deserialize)]
pub(crate) struct PitchingStats {
    pub(crate) stats: Vec<Stat<Pitcher>>
}

#[derive(Deserialize)]
pub(crate) struct Pitcher {
    pub(crate) inningsPitched: String,
    #[serde(default)]
    pub(crate) hits: i32,
    #[serde(default)]
    pub(crate) earnedRuns: i32,
    #[serde(default)]
    pub(crate) baseOnBalls: i32,
    #[serde(default)]
    pub(crate) strikeOuts: i32,
    #[serde(default)]
    pub(crate) hitByPitch: i32,
    #[serde(default)]
    pub(crate) wins: i32,
    #[serde(default)]
    pub(crate) losses: i32,
    #[serde(default)]
    pub(crate) winPercentage: String,
    #[serde(default)]
    pub(crate) era: String,
    #[serde(default)]
    pub(crate) avg: String,
    #[serde(default)]
    pub(crate) whip: String,
    #[serde(default)]
    pub(crate) obp: String,
    #[serde(default)]
    pub(crate) slg: String,
    #[serde(default)]
    pub(crate) ops: String,
    #[serde(default)]
    pub(crate) strikeoutsPer9Inn: String,
    #[serde(default)]
    pub(crate) walksPer9Inn: String,
    #[serde(default)]
    pub(crate) strikeoutWalkRatio: String,
    #[serde(default)]
    pub(crate) homeRunsPer9: String,
    #[serde(default)]
    pub(crate) saves: i32,
    #[serde(default)]
    pub(crate) saveOpportunities: i32
}

macro_rules! pitching_stats_url {
    () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats={}&group=pitching" };
}

macro_rules! pitching_header {
    ($col0:expr) => {
        row!($col0, "W", "L", "PCT", "ERA", "IP", "AVG", "WHIP", "OBP", "SLG", "OPS", "SO/9", "BB/9", "SO/BB", "HR/9", "SV", "SVO")
    };
}
pub(crate) use pitching_header;

macro_rules! pitching_row {
    ($col0:expr, $stat_group:expr) => {
        row!(
            $col0, $stat_group.wins, $stat_group.losses,
            &$stat_group.winPercentage, &$stat_group.era, &$stat_group.inningsPitched, &$stat_group.avg,
            &$stat_group.whip, &$stat_group.obp, &$stat_group.slg, &$stat_group.ops,
            &$stat_group.strikeoutsPer9Inn, &$stat_group.walksPer9Inn, &$stat_group.strikeoutWalkRatio,
            &$stat_group.homeRunsPer9, $stat_group.saves, $stat_group.saveOpportunities
        )
    };
}
pub(crate) use pitching_row;

pub(crate) fn get_season_pitching_stats(player_id: i32) -> reqwest::Result<PitchingStats> {
    reqwest::blocking::get(format!(pitching_stats_url!(), player_id, "season")).unwrap().json()
}

pub(crate) fn get_pitching_stats(player_id: i32, season_type: &str) -> PitchingStats {
    let url = format!(pitching_stats_url!(), player_id, season_type);
    reqwest::blocking::get(url).unwrap().json().unwrap()
}

pub(crate) fn get_pitching_row(stats: &PitchingStats) -> Row {
    let split = &stats.stats[0].splits[0];
    pitching_row!(&split.player.fullName, &split.stat)
}

pub(crate) fn display_pitching_stats(player_id: i32, season_type: &str) {
    let stats: PitchingStats = get_pitching_stats(player_id, season_type);

    let mut table = Table::new();
    table.add_row(pitching_header!("Year"));

    for stat in &stats.stats {
        for split in &stat.splits {
            table.add_row(pitching_row!(&split.season, &split.stat));
        }
    }

    println!("\nPlayer: {}\n\nPitching Statistics:\n{}", &stats.stats[0].splits[0].player.fullName, table.render());
}
