use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("{0}")]
    QueryTooShort(String),
    #[error("{0}")]
    GameError(String),
    #[error("No entry found for {0}")]
    EntryError(String),
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[error(transparent)]
    IOError(#[from] std::io::Error)
}

macro_rules! empty {
    () => { &"".to_string() };
}
pub(crate) use empty;

macro_rules! get_query_param {
    ($query:expr, $index:expr, $default:expr) => {
        $query.get($index).unwrap_or($default).to_ascii_lowercase()
    };
}
pub(crate) use get_query_param;
