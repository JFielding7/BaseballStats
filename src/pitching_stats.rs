use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PitchingStats {
    pub(crate) stats: (Stat,)
}

#[derive(Deserialize)]
pub(crate) struct Stat {
    pub(crate) splits: Vec<Split>
}

#[derive(Deserialize)]
pub(crate) struct Split {
    season: String,
    stat: PitcherStats
}

#[derive(Deserialize)]
pub(crate) struct PitcherStats {
    wins: i32,
    losses: i32,
    winPercentage: String,
    era: String,
    inningsPitched: i32,
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
