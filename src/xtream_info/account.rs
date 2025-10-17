use chrono::{DateTime, NaiveDateTime};
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Xdate {
    Str(String),
    I64(i64),
}

pub trait XdateExtensions {
    fn to_date(&self) -> NaiveDateTime;
}

impl XdateExtensions for Xdate {
    fn to_date(&self) -> NaiveDateTime {
        let ts = match self {
            Xdate::Str(s) => s.parse::<i64>().unwrap(),
            Xdate::I64(i) => *i,
        };
        DateTime::from_timestamp(ts, 0)
            .unwrap_or_default()
            .naive_utc()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Xbool {
    Str(String),
    Bool(bool),
}

pub trait XboolExtensions {
    fn to_bool(&self) -> bool;
}

impl XboolExtensions for Xbool {
    fn to_bool(&self) -> bool {
        match self {
            Xbool::Str(s) => matches!(s.as_str(), "1"),
            Xbool::Bool(b) => *b,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub created_at: Xdate,
    pub exp_date: Xdate,
    pub status: String,
    pub max_connections: String,
    pub active_cons: String,
    pub is_trial: Xbool,
}
