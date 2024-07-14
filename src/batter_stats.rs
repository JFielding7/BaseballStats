use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Statistics {
    stats: (Stat, AdvancedStat)
}

#[derive(Debug, Deserialize)]
struct Stat {
    splits: Split
}

#[derive(Debug, Deserialize)]
struct Split {
    stat: BatterStats
}

#[derive(Debug, Deserialize)]
struct AdvancedStat {
    splits: AdvancedSplit
}

#[derive(Debug, Deserialize)]
struct AdvancedSplit {
    stat: BatterAdvancedStats
}

#[derive(Debug, Deserialize)]
struct BatterStats {
    gamesPlayed: i32,
    runs: i32,
    doubles: i32,
    triples: i32,
    homeRuns: i32,
    strikeOuts: i32,
    baseOnBalls: i32,
    intentionalWalks: i32,
    hits: i32,
    hitByPitch: i32,
    avg: String,
    atBats: i32,
    obp: String,
    slg: String,
    ops: String,
    caughtStealing: i32,
    stolenBases: i32,
    stolenBasePercentage: String,
    groundIntoDoublePlay: i32,
    plateAppearances: i32,
    totalBases: i32,
    rbi: i32,
    leftOnBase: i32,
    sacBunts: i32,
    sacFlies: i32,
    atBatsPerHomeRun: String
}

#[derive(Debug)]
struct BatterAdvancedStats {
    pitchesPerPlateAppearance: String,
    walksPerPlateAppearance: String,
    strikeoutsPerPlateAppearance: String,
    homeRunsPerPlateAppearance: String,
    walksPerStrikeout: String
}