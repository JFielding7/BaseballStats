use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Statistics {
    stats: Vec<Stat>
}

#[derive(Debug, Deserialize)]
struct Stat {
    splits: Vec<Split>
}

#[derive(Debug, Deserialize)]
struct Split {
    stat: PlayerStats
}

#[derive(Debug, Deserialize)]
struct PlayerStats {
    gamesPlayed: i32,
    groundOuts: i32,
    airOuts: i32,
    runs: i32,
    doubles: i32,
    triples: i32,
    homeRuns: i32,
    strikeOuts: i32,
    baseOnBalls: i32,
    intentionalWalks: i32,
    hits: i32,
    hitByPitch: i32,
    avg: String
}

fn main() {
    let response = reqwest::blocking::get("https://statsapi.mlb.com/api/v1/people/630105/stats?stats=career&group=hitting&language=en").unwrap();
    let data: Statistics = response.json().unwrap();
    println!("{:?}", data);
}