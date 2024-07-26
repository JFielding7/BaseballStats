use std::cmp::max;
use figlet_rs::FIGfont;
use std::collections::HashMap;
use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use crate::hitting_stats::{Batter};
use crate::pitching_stats::{Pitcher};
use crate::{database, stats};

#[derive(Deserialize)]
struct Schedule {
    dates: Vec<Date>
}

#[derive(Deserialize)]
struct Date {
    games: Vec<Game>
}

#[derive(Deserialize)]
struct Game {
    gamePk: i32,
    teams: PlayingTeams,
    status: Status
}

#[derive(Deserialize)]
struct PlayingTeams {
    away: PlayingTeam,
    home: PlayingTeam
}

#[derive(Deserialize)]
struct PlayingTeam {
    leagueRecord: Record,
    score: i32,
    team: database::Team
}

#[derive(Deserialize)]
struct Status {
    abstractGameState: String
}

#[derive(Deserialize)]
struct LineScore {
    innings: Vec<Inning>,
    teams: TeamScores
}

#[derive(Deserialize)]
struct Inning {
    num: i32,
    home: Score,
    away: Score
}

#[derive(Deserialize)]
struct TeamScores {
    away: Score,
    home: Score
}

#[derive(Deserialize)]
struct Score {
    runs: Option<i32>,
    #[serde(default)]
    hits: i32,
    #[serde(default)]
    errors: i32,
    #[serde(default)]
    leftOnBase: i32
}

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
    team: TeamInfo,
    teamStats: Stats,
    players: HashMap<String, Player>
}

#[derive(Deserialize)]
struct Player {
    person: stats::Player,
    stats: Stats,
    seasonStats: Stats,
    #[serde(default)]
    battingOrder: String
}

#[derive(Deserialize)]
struct TeamInfo {
    name: String,
    abbreviation: String,
    record: Record
}

#[derive(Deserialize)]
struct Record {
    wins: i32,
    losses: i32,
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

macro_rules! games_today_url {
    () => { "https://statsapi.mlb.com/api/v1/schedule/games/?sportId=1" };
}

macro_rules! box_score_url {
    ($game_id:expr) => { format!("https://statsapi.mlb.com/api/v1/game/{}/boxscore", $game_id) };
}

macro_rules! line_score_url {
    ($game_id:expr) => { format!("https://statsapi.mlb.com/api/v1/game/{}/linescore", $game_id) };
}

macro_rules! hitting_header {
    () => { row!("Player", "PA", "AB", "R", "H", "2B", "3B", "HR", "RBI", "SO", "BB", "HBP", "LOB", "AVG", "OBP", "SLG", "OPS") };
}

macro_rules! hitting_row {
    ($col0:expr, $stats:expr, $season_stats:expr) => {
        row!(
            $col0, $stats.plateAppearances, $stats.atBats, $stats.runs, $stats.hits,
            $stats.doubles, $stats.triples, $stats.homeRuns, $stats.rbi, $stats.strikeOuts,
            $stats.baseOnBalls, $stats.hitByPitch, $stats.leftOnBase, &$season_stats.avg, &$season_stats.obp,
            &$season_stats.slg, &$season_stats.ops
        )
    };
}

macro_rules! pitching_header {
    () => { row!("Player", "IP", "H", "ER", "BB", "HBP", "SO", "ERA") };
}

macro_rules! pitching_row {
    ($col0:expr, $stats:expr, $season_stats:expr) => {
        row!(
            $col0, &$stats.inningsPitched, $stats.hits,
            $stats.earnedRuns, $stats.baseOnBalls, $stats.hitByPitch,
            $stats.strikeOuts, &$season_stats.era
        )
    };
}

// macro_rules! pitching_stats {
//     ($pitcher:expr) => {
//         ($pitcher.stats.pitching.as_ref().unwrap(), $pitcher.seasonStats.pitching.as_ref().unwrap())
//     };
// }

macro_rules! display_stat_table {
    ($team_stats:expr, $players:expr, $stat_type:ident, $header:ident, $row_generator:ident) => {{
        let mut stat_table = Table::new();
        stat_table.add_row($header!());
        $players.iter().for_each(|&player|
            stat_table.add_row($row_generator!(
            &player.person.fullName,
            player.stats.$stat_type.as_ref().unwrap(),
            player.seasonStats.$stat_type.as_ref().unwrap()))
        );
        stat_table.add_row($row_generator!("Team", $team_stats, $team_stats));
        println!("{}", stat_table.render());
    }};
}

macro_rules! winning_losing_pitchers {
    ($pitchers:expr, $winning_pitcher:expr, $losing_pitcher:expr) => {{
        for &pitcher in $pitchers {
            let season_stats = pitcher.seasonStats.pitching.as_ref().unwrap();
            if pitcher.stats.pitching.as_ref().unwrap().wins == 1 {
                $winning_pitcher = format!("{} ({}-{})", pitcher.person.fullName, season_stats.wins, season_stats.losses);
                break;
            }
            else if pitcher.stats.pitching.as_ref().unwrap().losses == 1 {
                $losing_pitcher = format!("{} ({}-{})", pitcher.person.fullName, season_stats.wins, season_stats.losses);
                break;
            }
        }
    }};
}

macro_rules! display_score {
    ($away_team:expr, $home_team:expr, $line_score:expr) => {
        println!("{}", FIGfont::standard().unwrap().convert(
            &format!(
                "{}    {} - {}    {}", $away_team.team.abbreviation, $line_score.teams.away.runs.unwrap(),
                $line_score.teams.home.runs.unwrap(), $home_team.team.abbreviation
            )[..]).unwrap()
        );
    };
}

macro_rules! display_record {
    ($team:expr) => {{
        println!("{}: {}-{}", $team.team.name, $team.team.record.wins, $team.team.record.losses);
    }};
}

fn display_team_stats(team: &Team, hitters: &Vec<&Player>, pitchers: &Vec<&Player>) {
    println!("{} Stats\n\nBatting", &team.team.name);
    display_stat_table!(team.teamStats.batting.as_ref().unwrap(), hitters, batting, hitting_header, hitting_row);

    println!("Pitching");
    display_stat_table!(team.teamStats.pitching.as_ref().unwrap(), pitchers, pitching, pitching_header, pitching_row);
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

    println!("\nWinning Pitcher: {}\nLosing Pitcher: {}\n", winning_pitcher, losing_pitcher);
}

fn display_line_score(line_score: &LineScore, away_team: &Team, home_team: &Team) {
    let mut innings = Table::new();
    let mut innings_header: Vec<String> = vec!["Team".to_string()];
    innings_header.append(&mut line_score.innings.iter().map(|inning| inning.num.to_string()).collect());
    innings_header.append(&mut vec!["R".to_string(), "H".to_string(), "E".to_string(), "LOB".to_string()]);
    innings.add_row(Row::new(innings_header));

    let mut away_scores: Vec<String> = vec![away_team.team.name.clone()];
    let mut home_scores: Vec<String> = vec![home_team.team.name.clone()];
    for inning in &line_score.innings {
        let away_score = &inning.away;
        let home_score = &inning.home;
        away_scores.push(away_score.runs.unwrap().to_string());
        let home_runs_scored = home_score.runs;
        if home_runs_scored.is_some() {
            home_scores.push(home_runs_scored.unwrap().to_string());
        }
        else {
            home_scores.push("X".to_string());
        }
    }
    let away_score = &line_score.teams.away;
    let home_score = &line_score.teams.home;
    away_scores.append(&mut vec![away_score.runs.unwrap().to_string(), away_score.hits.to_string(),
                                 away_score.errors.to_string(), away_score.leftOnBase.to_string()]);
    home_scores.append(&mut vec![home_score.runs.unwrap().to_string(), home_score.hits.to_string(),
                                 home_score.errors.to_string(), home_score.leftOnBase.to_string()]);

    innings.add_row(Row::new(away_scores));
    innings.add_row(Row::new(home_scores));

    println!("{}", innings.render());
}

pub(crate) fn display_game_stats(game_id: i32) {
    let box_score: BoxScore = get(box_score_url!(game_id)).unwrap().json().unwrap();
    let line_score: LineScore = get(line_score_url!(game_id)).unwrap().json().unwrap();

    let away_team = &box_score.teams.away;
    let home_team = &box_score.teams.home;
    let (away_hitters, away_pitchers) = hitters_and_pitchers(away_team);
    let (home_hitters, home_pitchers) = hitters_and_pitchers(home_team);

    display_score!(away_team, home_team, line_score);
    display_line_score(&line_score, away_team, home_team);
    display_record!(away_team);
    display_record!(home_team);
    display_winning_and_losing_pitchers(&away_pitchers, &home_pitchers);

    const DIVIDER_LEN: usize = 128;
    println!("{}", "-".repeat(DIVIDER_LEN));
    display_team_stats(away_team, &away_hitters, &away_pitchers);
    println!("{}", "-".repeat(DIVIDER_LEN));
    display_team_stats(home_team, &home_hitters, &home_pitchers);
}

const STATES: [&str; 4] = ["Final", "Live", "Preview", "Other"];
macro_rules! state_index {
    ($game:expr) => {
        STATES.iter().position(|&curr| curr == $game.status.abstractGameState).unwrap()
    };
}

macro_rules! update_max {
    ($team:expr, $max:expr) => {
        let record = &$team.leagueRecord;
        $max = max($max, $team.team.name.len() +
            record.wins.to_string().len() +
            record.losses.to_string().len()
        )
    };
}

pub(crate) fn display_games_today() {
    let schedule: Schedule = get(games_today_url!()).unwrap().json().unwrap();
    let mut games = &schedule.dates[0].games;
    games.sort_by(|game0, game1| state_index!(game0).cmp(&state_index!(game1)));

    let mut max_away_len = 0;
    let mut max_home_len = 0;
    for game in games {
        update_max!(&game.teams.away, max_away_len);
        update_max!(&game.teams.home, max_home_len);
    }

    for game in games {
        let away_team = &game.teams.away;
        let home_team = &game.teams.home;

        // print!("{} ({}-{}){}{} - ", away_team.team.name, away_team.leagueRecord.wins, away_team.leagueRecord.losses, away_team.score);
        // println!("{} {} ({}-{})    ({})", home_team.score, home_team.team.name, home_team.leagueRecord.wins, home_team.leagueRecord.losses, game.status.abstractGameState);
    }
}
