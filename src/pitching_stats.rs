use serde::Deserialize;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::data_id::get_id;
use crate::stats::{Stat};

#[derive(Deserialize)]
pub(crate) struct PitchingStats {
    pub(crate) stats: Vec<Stat<PitcherStats>>
}

#[derive(Deserialize)]
pub(crate) struct PitcherStats {
    wins: i32,
    losses: i32,
    winPercentage: String,
    era: String,
    inningsPitched: String,
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

fn get_hitting_stats(player_id: i32, season_type: &String) -> PitchingStats {
    let season;
    match &season_type[..] {
        "c" => season = "career",
        "y" => season = "yearByYear,career",
        _ => season = "season"
    }
    let url = format!("https://statsapi.mlb.com/api/v1/people/{}/stats?stats={}&group=pitching", player_id, season);
    reqwest::blocking::get(url).unwrap().json().unwrap()
}

fn display_stats(stats: PitchingStats) {
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

pub(crate) fn display_pitching_stats(query: &Vec<String>) {
    if query.len() == 1 {
        return;
    }
    let season_type: String;
    if query.len() > 2 {
        season_type = query[2].to_lowercase();
    }
    else {
        season_type = "s".to_string();
    }

    const ID_LEN: usize = 6;
    let id = get_id("database/player_ids.txt", &query[1], ID_LEN).unwrap();
    if id.is_positive() {
        display_stats(get_hitting_stats(id, &season_type));
    }
    else {
        println!("Invalid player!");
    }
}
