use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Stat<T> {
    pub(crate) splits: Vec<Split<T>>
}

#[derive(Deserialize)]
pub(crate) struct Split<T> {
    #[serde(default = "default_season")]
    pub(crate) season: String,
    pub(crate) player: Player,
    pub(crate) stat: T
}

#[derive(Deserialize)]
pub(crate) struct Player {
    pub(crate) fullName: String
}

fn default_season() -> String {
    "Career".to_string()
}
