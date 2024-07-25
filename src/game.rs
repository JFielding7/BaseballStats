use std::collections::HashMap;
use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::hitting_stats::{Batter};
use crate::pitching_stats::{Pitcher};
use crate::stats;

#[derive(Deserialize)]
struct BoxScore {
    teams: Teams
}

#[derive(Deserialize)]
struct Teams {
    away: Team,
    home: Team,
}

#[derive(Deserialize)]
struct Team {
    team: TeamName,
    players: HashMap<String, Player>
}

#[derive(Deserialize)]
struct Player {
    person: stats::Player,
    stats: Stats,
    #[serde(default)]
    battingOrder: String
}

#[derive(Deserialize)]
struct TeamName {
    teamName: String,
    abbreviation: String
}

#[derive(Deserialize)]
struct Stats {
    #[serde(deserialize_with = "deserialize_stats")]
    batting: Option<Batter>,
    #[serde(deserialize_with = "deserialize_stats")]
    pitching: Option<Pitcher>
}

fn deserialize_stats<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where T: serde::Deserialize<'de>, D: serde::Deserializer<'de> {
    Ok(Option::<T>::deserialize(deserializer).unwrap_or(None))
}

macro_rules! box_score_url {
    ($game_id:expr) => { format!("https://statsapi.mlb.com/api/v1/game/{}/boxscore", $game_id) };
}

macro_rules! hitting_header {
    () => { row!("Player", "PA", "AB", "R", "H", "2B", "3B", "HR", "RBI", "SO", "BB", "HBP") };
}

macro_rules! hitting_row {
    ($hitter:expr) => {{
        let stat_group = $hitter.stats.batting.as_ref().unwrap();
        row!(
            &$hitter.person.fullName, stat_group.plateAppearances, stat_group.atBats,
            stat_group.runs, stat_group.hits, stat_group.doubles, stat_group.triples,
            stat_group.homeRuns, stat_group.rbi, stat_group.strikeOuts, stat_group.baseOnBalls,
            stat_group.hitByPitch
        )
    }};
}

macro_rules! pitching_header {
    () => { row!("Player", "IP", "H", "ER", "BB", "HBP", "SO") };
}

macro_rules! pitching_row {
    ($pitcher:expr) => {{
        let stat_group = $pitcher.stats.pitching.as_ref().unwrap();
        row!(
            &$pitcher.person.fullName, &stat_group.inningsPitched, stat_group.hits,
            stat_group.earnedRuns, stat_group.baseOnBalls, stat_group.hitByPitch,
            stat_group.strikeOuts
        )
    }};
}

macro_rules! display_stat_table {
    ($players:expr, $header:ident, $row_generator:ident) => {{
        let mut stat_table = Table::new();
        stat_table.add_row($header!());
        $players.iter().for_each(|&player| stat_table.add_row($row_generator!(player)));
        println!("{}", stat_table.render());
    }};
}

macro_rules! winning_losing_pitchers {
    ($pitchers:expr, $winning_pitcher:expr, $losing_pitcher:expr) => {{
        for &pitcher in $pitchers {
            if pitcher.stats.pitching.as_ref().unwrap().wins == 1 {
                $winning_pitcher = pitcher.person.fullName.clone();
            }
            else if pitcher.stats.pitching.as_ref().unwrap().losses == 1 {
                $losing_pitcher = pitcher.person.fullName.clone();
            }
        }
    }};
}

fn display_team_stats(team: &Team, hitters: &Vec<&Player>, pitchers: &Vec<&Player>) {
    println!("{} Stats\n\nBatting", &team.team.teamName);
    display_stat_table!(hitters, hitting_header, hitting_row);

    println!("Pitching");
    display_stat_table!(pitchers, pitching_header, pitching_row);
}

fn hitters_and_pitchers(team: &Team) -> (Vec<&Player>, Vec<&Player>) {
    let mut players: Vec<&Player> = team.players.values().collect();
    let (mut hitters, mut pitchers): (Vec<&Player>, Vec<&Player>) = players.iter().partition(|&&player| player.stats.batting.is_some());
    hitters.sort_by(|&player0, &player1| player0.battingOrder.cmp(&player1.battingOrder));

    let mut pitchers: Vec<&Player> = pitchers.into_iter().filter(|&player| player.stats.pitching.is_some()).collect();
    pitchers.sort_by(|&player0, &player1| {
        player1.stats.pitching.as_ref().unwrap().inningsPitched
            .cmp(&player0.stats.pitching.as_ref().unwrap().inningsPitched)
    });
    (hitters, pitchers)
}

fn display_winning_and_losing_pitchers(away_pitchers: &Vec<&Player>, home_pitchers: &Vec<&Player>) {
    let mut winning_pitcher = "".to_string();
    let mut losing_pitcher = "".to_string();

    winning_losing_pitchers!(away_pitchers, winning_pitcher, losing_pitcher);
    winning_losing_pitchers!(home_pitchers, winning_pitcher, losing_pitcher);

    println!("Winning Pitcher: {}\nLosing Pitcher: {}\n", winning_pitcher, losing_pitcher);
}

pub(crate) fn display_game_stats(game_id: i32) {
    let box_score: BoxScore = get(box_score_url!(game_id)).unwrap().json().unwrap();

    let (away_hitters, away_pitchers) = hitters_and_pitchers(&box_score.teams.away);
    let (home_hitters, home_pitchers) = hitters_and_pitchers(&box_score.teams.home);

    display_winning_and_losing_pitchers(&away_pitchers, &home_pitchers);

    display_team_stats(&box_score.teams.away, &away_hitters, &away_pitchers);
    const DIVIDER_LEN: usize = 100;
    println!("{}", "-".repeat(DIVIDER_LEN));
    display_team_stats(&box_score.teams.home, &home_hitters, &home_pitchers);
}
