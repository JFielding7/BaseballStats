use figlet_rs::FIGfont;
use std::collections::HashMap;
use std::{mem};
use chrono::{Datelike, Utc};
use serde::Deserialize;
use reqwest::blocking::get;
use term_table::{row, rows, Table};
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::TableStyle;
use crate::hitting_stats::{Batter};
use crate::pitching_stats::{Pitcher};
use crate::{database, stats};
use crate::query::{empty, get_query_param, QueryError};
use crate::teams::get_team;

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
    gameDate: String,
    officialDate: String,
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
    #[serde(default)]
    score: i32,
    team: database::Team
}

#[derive(Deserialize)]
struct Status {
    abstractGameState: String,
    detailedState: String
}

#[derive(Deserialize)]
struct Feed {
    gameData: Data,
    liveData: LiveData
}

#[derive(Deserialize)]
struct Data {
    datetime: DateTime,
    status: Status
}

#[derive(Deserialize)]
struct DateTime {
    dateTime: String,
}

#[derive(Deserialize)]
struct LiveData {
    plays: Plays,
    linescore: LineScore,
}

#[derive(Deserialize)]
struct Plays {
    currentPlay: Option<Play>
}

#[derive(Deserialize)]
struct Play {
    runners: Vec<Runner>
}

#[derive(Deserialize)]
struct Runner {
    movement: Movement
}

#[derive(Deserialize)]
struct Movement {
    end: Option<String>
}

#[derive(Deserialize)]
struct LineScore {
    innings: Vec<Inning>,
    teams: TeamScores,
    #[serde(default)]
    inningState: String,
    #[serde(default)]
    currentInningOrdinal: String,
    #[serde(default)]
    outs: i32,
    #[serde(default)]
    balls: i32,
    #[serde(default)]
    strikes: i32
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
struct WinProbability {
    homeTeamWinProbability: f64,
    awayTeamWinProbability: f64
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

macro_rules! games_url {
    ($queries:expr) => { format!("https://statsapi.mlb.com/api/v1/schedule/games/?sportId=1{}", $queries) };
}

macro_rules! box_score_url {
    ($game_id:expr) => { format!("https://statsapi.mlb.com/api/v1/game/{}/boxscore", $game_id) };
}

macro_rules! line_score_url {
    ($game_id:expr) => { format!("https://statsapi.mlb.com/api/v1/game/{}/linescore", $game_id) };
}

macro_rules! game_feed_url {
    ($game_id:expr) => { format!("https://statsapi.mlb.com/api/v1.1/game/{}/feed/live", $game_id) };
}

macro_rules! season_games_url {
    ($team_id:expr, $season:expr) => {
        format!("https://statsapi.mlb.com/api/v1/schedule?sportId=1&teamId={}&season={}", $team_id, $season)
    };
}

macro_rules! win_probability_url {
    ($team_id:expr) => {
        format!("https://statsapi.mlb.com/api/v1/game/{}/winProbability", $team_id)
    }
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
            format!(
                "{}    {} - {}    {}", $away_team.team.abbreviation, $line_score.teams.away.runs.unwrap_or(0),
                $line_score.teams.home.runs.unwrap_or(0), $home_team.team.abbreviation
            ).as_str()).unwrap()
        );
    };
}

macro_rules! display_record {
    ($team:expr) => {{
        println!("{}: {}-{}", $team.team.name, $team.team.record.wins, $team.team.record.losses);
    }};
}

macro_rules! upcoming_game {
    ($away_team:expr, $away_record:expr, $home_team:expr, $home_record:expr, $feed:expr, $middle0:expr, $middle1:expr, $middle2:expr) => {{
        let mut row = vec![
            format!("{} ({}-{})", $away_team.team.name, $away_record.wins, $away_record.losses),
            $middle0.to_string(), $middle1.to_string(), $middle2.to_string(),
            format!("{} ({}-{})", $home_team.team.name, $home_record.wins, $home_record.losses),
        ];
        row.append(&mut get_game_state(&$feed));
        Row::new(row)
    }};
}

fn get_eastern_standard_time(date_time: &String) -> String {
    const EST_OFFSET: i32 = 20;
    const HOURS: i32 = 12;

    let colon_index = date_time.find(":").unwrap();
    let hour_24 = (date_time[colon_index - 2..colon_index].parse::<i32>().unwrap() + EST_OFFSET) % 24;
    let hour_12 = (hour_24 + HOURS - 1) % HOURS + 1;
    let minutes = &date_time[colon_index..colon_index + 3];
    let mut time_of_day = "PM";
    if hour_24 < 12 {
        time_of_day = "AM";
    }
    format!("{}{}{}", hour_12, minutes, time_of_day)
}

fn get_game_state(feed: &Feed) -> Vec<String> {
    match feed.gameData.status.abstractGameState.as_str() {
        "Final" => vec!["Final".to_string()],
        "Live" => {
            let line_score = &feed.liveData.linescore;
            let bases: Vec<String> = feed.liveData.plays.currentPlay.as_ref()
                .unwrap_or(&Play { runners: Vec::new() }).runners
                .iter().filter_map(|runner| {
                match &runner.movement.end {
                    Some(bases) => Some(bases.clone()),
                    None => None
                }
            }).collect();
            vec![
                format!("{} {}", line_score.inningState, line_score.currentInningOrdinal),
                format!("{}-{}", line_score.balls, line_score.strikes),
                format!("{} Out", line_score.outs), format!("Bases: {}", bases.join(", "))
            ]
        },
        "Preview" => vec![get_eastern_standard_time(&feed.gameData.datetime.dateTime)],
        _ => vec!["".to_string()]
    }
}

fn display_team_stats(team: &Team, hitters: &Vec<&Player>, pitchers: &Vec<&Player>) {
    println!("{} Stats\n\nBatting", &team.team.name);
    display_stat_table!(team.teamStats.batting.as_ref().unwrap(), hitters, batting, hitting_header, hitting_row);

    println!("Pitching");
    display_stat_table!(team.teamStats.pitching.as_ref().unwrap(), pitchers, pitching, pitching_header, pitching_row);
}

fn hitters_and_pitchers(team: &Team) -> (Vec<&Player>, Vec<&Player>) {
    let players: Vec<&Player> = team.players.values().collect();
    let (mut hitters, pitchers): (Vec<&Player>, Vec<&Player>) = players
        .iter().partition(|&&player| player.stats.batting.is_some());
    hitters.sort_by(|&player0, &player1| player0.battingOrder.cmp(&player1.battingOrder));

    let mut pitchers: Vec<&Player> = pitchers.
        into_iter().filter(|&player| player.stats.pitching.is_some()).collect();
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

fn get_final_score(team_score: &Score) -> Vec<String> {
    vec![team_score.runs.unwrap_or(0).to_string(), team_score.hits.to_string(),
         team_score.errors.to_string(), team_score.leftOnBase.to_string()]
}

fn display_line_score(game_id: i32, feed: &Feed, line_score: &LineScore, away_team: &Team, home_team: &Team) -> reqwest::Result<()> {
    const SCHEDULED_INNINGS: i32 = 9;

    let mut innings = Table::new();
    let mut innings_header: Vec<String> = vec!["Team".to_string()];
    innings_header.append(&mut line_score.innings.iter().map(|inning| inning.num.to_string()).collect());

    let mut away_scores: Vec<String> = vec![away_team.team.name.clone()];
    let mut home_scores: Vec<String> = vec![home_team.team.name.clone()];
    for inning in &line_score.innings {
        let away_score = &inning.away;
        let away_score_option = away_score.runs;
        if away_score_option.is_some() {
            away_scores.push(away_score_option.unwrap_or(0).to_string());
        }
        else {
            away_scores.push(" ".to_string());
        }

        let home_score = &inning.home;
        let home_score_option = home_score.runs;
        if home_score_option.is_some() {
            home_scores.push(home_score_option.unwrap_or(0).to_string());
        }
        else if inning.num == SCHEDULED_INNINGS && &feed.gameData.status.abstractGameState == "Final" {
            home_scores.push("X".to_string());
        }
        else {
            home_scores.push(" ".to_string());
        }
    }
    for i in (line_score.innings.len() + 1)..10 {
        innings_header.push(i.to_string());
        away_scores.push(" ".to_string());
        home_scores.push(" ".to_string());
    }
    away_scores.append(&mut get_final_score(&line_score.teams.away));
    home_scores.append(&mut get_final_score(&line_score.teams.home));

    innings_header.append(&mut vec!["R".to_string(), "H".to_string(), "E".to_string(), "LOB".to_string()]);
    innings.add_row(Row::new(innings_header));
    innings.add_row(Row::new(away_scores));
    innings.add_row(Row::new(home_scores));

    println!(
        "{}\n{}",
        Table::builder().rows(vec![Row::new(get_game_state(&feed))]).build().render(),
        innings.render()
    );
    if &feed.gameData.status.abstractGameState == "Live" {
        display_win_probability(game_id, away_team, home_team)?;
    }
    Ok(())
}

fn display_games(games: &Vec<Game>) -> reqwest::Result<()> {
    let mut game_feeds: Vec<(&Game, Feed)> = Vec::with_capacity(games.len());
    for game in games {
        let feed: Feed = get(game_feed_url!(game.gamePk))?.json()?;
        game_feeds.push((game, feed));
    }

    let mut game_table = Table::new();
    game_table.style = TableStyle::blank();
    for (game, feed) in game_feeds {
        let teams = &game.teams;

        let away_team = &teams.away;
        let away_record = &away_team.leagueRecord;
        let home_team = &teams.home;
        let home_record = &home_team.leagueRecord;
        let game_state = &game.status.abstractGameState;

        if game_state == "Final" || game_state == "Live" {
            // let mut row = vec![
            //     format!("{} ({}-{})", away_team.team.name, away_record.wins, away_record.losses),
            //     away_team.score.to_string(), "-".to_string(), home_team.score.to_string(),
            //     format!("{} ({}-{})", home_team.team.name, home_record.wins, home_record.losses)
            // ];
            // row.append(&mut get_game_state(&feed));
            // game_table.add_row(Row::new(row));
            game_table.add_row(upcoming_game!(away_team, away_record, home_team, home_record, feed, away_team.score, "-", home_team.score))
        }
        else {
            game_table.add_row(upcoming_game!(away_team, away_record, home_team, home_record, feed, "", "@", ""));
        }
    }
    println!("{}", game_table.render());
    Ok(())
}

fn display_win_probability(game_id: i32, away_team: &Team, home_team: &Team) -> reqwest::Result<()> {
    let win_probability: Vec<WinProbability> = get(win_probability_url!(game_id))?.json()?;
    let current_probability = &win_probability[win_probability.len() - 1];
    println!(
        "Win Probability:\n{}: {:.1}%\n{}: {:.1}%\n", away_team.team.name,
        current_probability.awayTeamWinProbability, home_team.team.name,
        current_probability.homeTeamWinProbability
    );
    Ok(())
}

pub(crate) fn display_game_stats(game_id: i32) -> reqwest::Result<()> {
    let box_score: BoxScore = get(box_score_url!(game_id))?.json()?;
    let line_score: LineScore = get(line_score_url!(game_id))?.json()?;
    let feed: Feed = get(game_feed_url!(game_id))?.json()?;

    let away_team = &box_score.teams.away;
    let home_team = &box_score.teams.home;
    let (away_hitters, away_pitchers) = hitters_and_pitchers(away_team);
    let (home_hitters, home_pitchers) = hitters_and_pitchers(home_team);

    if feed.gameData.status.abstractGameState == "Preview" {
        let away_record = &away_team.team.record;
        let home_record = &home_team.team.record;
        println!("{}", Table::builder().style(TableStyle::blank())
            .rows(rows![upcoming_game!(away_team, away_record, home_team, home_record, feed, "", "@", "")]).build().render()
        );
    }
    else {
        display_score!(away_team, home_team, line_score);
        display_line_score(game_id, &feed, &line_score, away_team, home_team)?;
        display_record!(away_team);
        display_record!(home_team);
        display_winning_and_losing_pitchers(&away_pitchers, &home_pitchers);

        const DIVIDER_LEN: usize = 128;
        println!("{}", "-".repeat(DIVIDER_LEN));
        display_team_stats(away_team, &away_hitters, &away_pitchers);
        println!("{}", "-".repeat(DIVIDER_LEN));
        display_team_stats(home_team, &home_hitters, &home_pitchers);
    }
    Ok(())
}

pub(crate) fn display_games_today() -> reqwest::Result<()> {
    let mut schedule: Schedule = get(games_url!(""))?.json()?;
    display_games(&mut schedule.dates[0].games)?;
    Ok(())
}

fn filter_games(schedule: Schedule, predicate: fn(&Game) -> bool) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for date in schedule.dates {
        for game in date.games {
            if predicate(&game) {
                games.push(game);
            }
        }
    }
    games
}

fn get_team_and_opp(team_id: i32, game: &Game) -> (&PlayingTeam, &PlayingTeam, &str) {
    let teams = &game.teams;
    let mut opp = &teams.away;
    let mut team = &teams.home;
    let mut symbol = "vs";

    if opp.team.id == team_id {
        mem::swap(&mut team, &mut opp);
        symbol = "@ ";
    }
    (team, opp, symbol)
}

fn display_past_games(team_id: i32, limit: usize) -> reqwest::Result<()> {
    let schedule: Schedule = get(season_games_url!(team_id,  Utc::now().year()))?.json()?;
    let games: Vec<Game> = filter_games(schedule, |game| &game.status.detailedState == "Final");
    let mut start = 0;
    if limit < games.len() {
        start = games.len() - limit;
    }

    let mut game_results = Table::new();
    game_results.style = TableStyle::blank();
    game_results.add_row(row!("Opponent", "Opp Record", "Result", "Record"));

    for game in &games[start..] {
        let (team, opp, symbol) = get_team_and_opp(team_id, game);

        let mut res = "W";
        if team.score < opp.score {
            res = "L";
        }

        game_results.add_row(row!(
            format!("{} {}", symbol, opp.team.name),
            format!("({}-{})", opp.leagueRecord.wins, opp.leagueRecord.losses),
            format!("{} {}-{}", res, team.score, opp.score),
            format!("({}-{})", team.leagueRecord.wins, team.leagueRecord.losses)
        ));
    }
    println!("{}", game_results.render());
    Ok(())
}

fn display_schedule(team_id: i32, limit: usize) -> reqwest::Result<()> {
    let schedule: Schedule = get(season_games_url!(team_id,  Utc::now().year()))?.json()?;
    let games: Vec<Game> = filter_games(schedule, |game| &game.status.abstractGameState == "Preview");
    let upcoming_games: Vec<Game> = games.into_iter().take(limit).collect();

    let mut schedule_table = Table::new();
    schedule_table.style = TableStyle::blank();

    for game in &upcoming_games {
        let (_team, opp, symbol) = get_team_and_opp(team_id, game);

        schedule_table.add_row(row!(
            format!("{} {}", symbol, opp.team.name),
            format!("({}-{})", opp.leagueRecord.wins, opp.leagueRecord.losses),
            format!("{}", &game.officialDate),
            format!("{}", get_eastern_standard_time(&game.gameDate))
        ));
    }
    println!("{}", schedule_table.render());
    Ok(())
}

fn get_game_id(team: &String, date: &String) -> Result<i32, QueryError> {
    let (_, team_id) = get_team(team)?;

    let today: bool;
    let schedule: Schedule = if date.is_empty() {
        today = true;
        get(games_url!(format!("&teamId={team_id}")))?.json()?
    }
    else {
        today = false;
        get(games_url!(format!("&teamId={team_id}&startDate={date}&endDate={date}")))?.json()?
    };
    if schedule.dates.len() > 0 {
       return Ok(schedule.dates[0].games[0].gamePk);
    }
    let error_message = if today {
        format!("No games for {team} today")
    }
    else {
        format!("No games for {team} on {date}")
    };
    Err(QueryError::GameError(error_message))
}

pub(crate) fn games_query(query: &Vec<String>) -> Result<(), QueryError> {
    const TEAM_INDEX: usize = 2;
    const DATE_INDEX: usize = 3;

    let default = empty!();
    let team = get_query_param!(query, TEAM_INDEX, default);
    match team.as_str() {
        "" => {
            display_games_today()?;
            Ok(())
        },
        _ => {
            let date = query.get(DATE_INDEX).unwrap_or(default);
            let game_id = get_game_id(&team, date)?;
            display_game_stats(game_id)?;
            Ok(())
        }
    }
}

pub(crate) fn season_games_query(query: &Vec<String>) -> Result<(), QueryError> {
    const TYPE_INDEX: usize = 1;
    const TEAM_INDEX: usize = 2;
    const LIMIT_INDEX: usize = 3;
    const DEFAULT_LIMIT: usize = 8;

    let default = empty!();
    let (_, team_id) = get_team(&get_query_param!(query, TEAM_INDEX, default))?;
    let limit = &query.get(LIMIT_INDEX).unwrap_or(&DEFAULT_LIMIT.to_string()).parse::<usize>().unwrap_or(DEFAULT_LIMIT);

    match query[TYPE_INDEX].to_ascii_lowercase().as_str() {
        "r" | "results" => display_past_games(team_id, *limit)?,
        _ => display_schedule(team_id, *limit)?
    }
    Ok(())
}
