use serde::Deserialize;
use reqwest::blocking::{get};
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::query::QueryError;

#[derive(Deserialize)]
struct Standings {
    records: Vec<Division>
}

#[derive(Deserialize)]
struct Division {
    teamRecords: Vec<Team>
}

#[derive(Deserialize)]
struct Team {
    team: TeamName,
    streak: Streak,
    leagueRecord: Record,
    gamesBack: String,
    wildCardGamesBack: String,
    runsScored: i32,
    runsAllowed: i32,
    runDifferential: i32,
    records: Records
}

#[derive(Deserialize)]
struct Records {
    overallRecords: Vec<Record>,
    expectedRecords: Vec<Record>,
    splitRecords: Vec<Record>
}

#[derive(Deserialize)]
struct Record {
    wins: u8,
    losses: u8,
    pct: String
}

#[derive(Deserialize)]
struct TeamName {
    name: String
}

#[derive(Deserialize)]
struct Streak {
    streakCode: String
}

macro_rules! locations {
    () => { vec!["East              ", "Central           ", "West              "] };
}

macro_rules! division_header {
    ($league:expr, $location:expr) => {
        row!(format!("{} {}", $league, $location), "W", "L", "PCT", "GB", "WCGB", "L10", "STRK",
        "RS", "RA", "DIFF", "X W-L", "XS W-L", "HOME", "AWAY", ">.500")
    };
}

fn display_league_standings(league: &str, standings: &Standings) {
    const EXPECTED_CURR_INDEX: usize = 0;
    const EXPECTED_SEASON_INDEX: usize = 1;
    const HOME_RECORD_INDEX: usize = 0;
    const AWAY_RECORD_INDEX: usize = 1;
    const LAST_TEN_INDEX: usize = 8;
    const WINNING_TEAMS_INDEX: usize = 11;

    let locations = locations!();
    let arr: Vec<(&Division, &&str)> = standings.records.iter().zip(locations.iter()).collect();
    for (division, location) in arr {
        let mut table = Table::new();
        table.add_row(division_header!(league, location));
        for team in &division.teamRecords {
            let expected_records = &team.records.expectedRecords;
            let expected_curr = &expected_records[EXPECTED_CURR_INDEX];
            let expected_season = &expected_records[EXPECTED_SEASON_INDEX];

            let ballpark_records = &team.records.overallRecords;
            let home = &ballpark_records[HOME_RECORD_INDEX];
            let away = &ballpark_records[AWAY_RECORD_INDEX];

            let split_records = &team.records.splitRecords;
            let last_ten = &split_records[LAST_TEN_INDEX];
            let winning_record_teams = &split_records[WINNING_TEAMS_INDEX];

            table.add_row(row!(
                &team.team.name, team.leagueRecord.wins, team.leagueRecord.losses,
                &team.leagueRecord.pct, &team.gamesBack, &team.wildCardGamesBack,
                format!("{}-{}", last_ten.wins, last_ten.losses), &team.streak.streakCode,
                team.runsScored, team.runsAllowed, team.runDifferential,
                format!("{}-{}", expected_curr.wins, expected_curr.losses),
                format!("{}-{}", expected_season.wins, expected_season.losses),
                format!("{}-{}", home.wins, home.losses), format!("{}-{}", away.wins, away.losses),
                format!("{}-{}", winning_record_teams.wins, winning_record_teams.losses)
            ));
        }
        println!("{}", table.render());
    }
}

pub(crate) fn display_standings() -> Result<(), QueryError> {
    const AL_URL: &str = "https://statsapi.mlb.com/api/v1/standings?leagueId=103";
    const NL_URL: &str = "https://statsapi.mlb.com/api/v1/standings?leagueId=104";

    let nl_standings: Standings = get(NL_URL)?.json()?;
    let al_standings: Standings = get(AL_URL)?.json()?;

    println!("\nMLB Standings\n\nNational League\n");
    display_league_standings("NL", &nl_standings);

    println!("\nAmerican League\n");
    display_league_standings("AL", &al_standings);
    Ok(())
}
