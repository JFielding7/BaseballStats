use serde::Deserialize;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::stats::{Stat};

#[derive(Deserialize)]
pub(crate) struct PitchingStats {
    pub(crate) stats: Vec<Stat<PitcherStats>>
}

#[derive(Deserialize)]
pub(crate) struct PitcherStats {
    pub(crate) inningsPitched: String,
    wins: i32,
    losses: i32,
    winPercentage: String,
    era: String,
    avg: String,
    whip: String,
    obp: String,
    slg: String,
    ops: String,
    strikeoutsPer9Inn: String,
    walksPer9Inn: String,
    strikeoutWalkRatio: String,
    homeRunsPer9: String,
    saves: i32,
    saveOpportunities: i32
}

macro_rules! pitching_stats_url {
    () => { "https://statsapi.mlb.com/api/v1/people/{}/stats?stats={}&group=pitching" };
}

pub(crate) fn get_pitching_stats(player_id: i32, season_type: &str) -> PitchingStats {
    let url = format!(pitching_stats_url!(), player_id, season_type);
    reqwest::blocking::get(url).unwrap().json().unwrap()
}

pub(crate) fn display_pitching_stats(player_id: i32, season_type: &str) {
    let stats: PitchingStats = get_pitching_stats(player_id, season_type);

    let mut table = Table::new();
    table.add_row(row!("Year", "W", "L", "W/L", "ERA", "IP", "AVG", "WHIP", "OBP", "SLG", "OPS", "SO/9", "BB/9", "SO/BB", "HR/9", "SV", "SVO"));

    for stat in &stats.stats {
        for split in &stat.splits {
            let stat_group = &split.stat;
            table.add_row(row!(&split.season, stat_group.wins, stat_group.losses,
                &stat_group.winPercentage, &stat_group.era, &stat_group.inningsPitched, &stat_group.avg,
                &stat_group.whip, &stat_group.obp, &stat_group.slg, &stat_group.ops,
                &stat_group.strikeoutsPer9Inn, &stat_group.walksPer9Inn, &stat_group.strikeoutWalkRatio,
                &stat_group.homeRunsPer9, stat_group.saves, stat_group.saveOpportunities)
            );
        }
    }

    println!("\nPlayer: {}\n\nPitching Statistics:\n{}", &stats.stats[0].splits[0].player.fullName, table.render());
}
