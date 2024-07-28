use reqwest::blocking::get;
use serde::Deserialize;
use term_table::{row, Table, TableBuilder, TableStyle, rows};
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};

use phf;
use phf_macros::phf_map;

const HEADER_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "battingAverage" => "Batting Average Leaders",
    "homeRuns" => "Home Run Leaders",
    "runsBattedIn" => "RBI Leaders",
    "hits" => "Hit Leaders",
    "stolenBases" => "Stolen Base Leaders",
    "wins" => "Wins Leaders",
    "earnedRunAverage" => "ERA Leaders",
    "strikeouts" => "Strikeout Leaders",
    "walksAndHitsPerInningPitched" => "WHIP Leaders",
    "saves" => "Saves Leaders",
};

#[derive(Deserialize)]
struct Leaders {
    leagueLeaders: Vec<Category>
}

#[derive(Deserialize)]
struct Category {
    leaderCategory: String,
    leaders: Vec<Player>
}

#[derive(Deserialize)]
struct Player {
    rank: i32,
    value: String,
    team: Team,
    person: Person
}

#[derive(Deserialize)]
struct Team {
    name: String
}

#[derive(Deserialize)]
struct Person {
    fullName: String
}

enum Stats {
    ALL,
    Batting,
    Pitching,
    Stat(&'static str, &'static str)
}

macro_rules! leaders_url {
    ($categories:expr, $group:expr, $limit:expr) => {
        format!("https://statsapi.mlb.com/api/v1/stats/leaders?leaderCategories={}&statGroup={}&limit={}", $categories, $group, $limit);
    };
}

macro_rules! display_batting_leaders {
    ($limit:expr) => {{
        println!("\n{}", Table::builder().rows(rows![row!["Batting Leaders"]]).build().render());
        display_leader_stats("battingAverage,homeRuns,runsBattedIn,hits,stolenBases", "hitting", $limit);
    }};
}

macro_rules! display_pitching_leaders {
    ($limit:expr) => {{
        println!("\n{}", Table::builder().rows(rows![row!["Pitching Leaders"]]).build().render());
        display_leader_stats("wins,era,strikeOuts,whip,saves", "pitching", $limit);
    }};
}

fn display_leader_stats(categories: &str, group: &str, limit: i32) {
    let leaders: Leaders = get(leaders_url!(categories, group, limit)).unwrap().json().unwrap();
    for category in leaders.leagueLeaders {
        let mut table = Table::new();
        table.add_row(row!(TableCell::builder(HEADER_MAP.get(category.leaderCategory.as_str()).unwrap()).col_span(4).alignment(Alignment::Center).build()));
        table.add_row(row!("Rank", "Player", "Team", "Stat"));
        for leader in category.leaders {
            table.add_row(row!(leader.rank, leader.person.fullName, leader.team.name, leader.value));
        }
        println!("{}", table.render());
    }
}

fn display_stat_leaders(stats: Stats, limit: i32) {
    match stats {
        Stats::ALL => {
            display_batting_leaders!(limit);
            display_pitching_leaders!(limit);
        },
        Stats::Batting => display_batting_leaders!(limit),
        Stats::Pitching => display_pitching_leaders!(limit),
        Stats::Stat(category, group) => display_leader_stats(category, group, limit)
    }
}

pub(crate) fn display_leaders(query: &Vec<String>) {
    const PARAM_INDEX: usize = 2;
    const LIMIT_INDEX: usize = 3;
    const DEFAULT_LIMIT: i32 = 8;

    let stats= match query.get(PARAM_INDEX).unwrap_or(&"".to_string()).to_lowercase().as_str() {
        "b" => Stats::Batting,
        "p" => Stats::Pitching,
        "avg" => Stats::Stat("battingAverage", "hitting"),
        "hr" => Stats::Stat("homeRuns", "hitting"),
        "rbi" => Stats::Stat("runsBattedIn", "hitting"),
        "h" => Stats::Stat("hits", "hitting"),
        "sb" => Stats::Stat("stolenBases", "hitting"),
        "wins" => Stats::Stat("wins", "pitching"),
        "era" => Stats::Stat("era", "pitching"),
        "saves" => Stats::Stat("saves", "pitching"),
        "so" => Stats::Stat("strikeOuts", "pitching"),
        "whip" => Stats::Stat("whip", "pitching"),
        _ => Stats::ALL
    };

    let limit: i32 = if query.len() > LIMIT_INDEX {
        query[LIMIT_INDEX].parse::<i32>().unwrap_or(DEFAULT_LIMIT)
    }
    else { DEFAULT_LIMIT };

    display_stat_leaders(stats, limit);
}
