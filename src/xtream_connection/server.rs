use reqwest;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Server<'a> {
    server: &'a str,
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Xdate {
    Str(String),
    I64(i64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Xbool {
    Str(String),
    Bool(bool),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserInfo {
    pub user_info: Account,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Account {
    pub created_at: Xdate,
    pub exp_date: Xdate,
    pub status: String,
    pub max_connections: String,
    pub active_cons: String,
    pub is_trial: Xbool,
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

impl Server<'_> {
    async fn get_url<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<T, reqwest::Error> {
        match reqwest::get(url).await {
            Ok(resp) => {
                if resp.status() != 200 {
                    println!("Error {} getting {url}", resp.status());
                    panic!("Verify that your username and password are correct");
                }
                resp.json::<T>().await
            }
            Err(e) => {
                println!("Error: {e:?}");
                std::process::exit(1);
            }
        }
    }

    async fn get_vec_url(&self, url: &str) -> Result<Vec<Value>, reqwest::Error> {
        match reqwest::get(url).await {
            Ok(resp) => {
                if resp.status() != 200 {
                    println!("Error {} getting {url}", resp.status());
                }
                resp.json().await
            }
            Err(e) => Err(e),
        }
    }
    pub async fn get_account_info(&self) -> Account {
        let url = format!(
            "{}/player_api.php?username={}&password={}",
            self.server, self.username, self.password
        );
        match self.get_url::<UserInfo>(&url).await {
            Ok(r) => r.user_info,
            Err(e) => {
                println!("error {e:?}");
                std::process::exit(1);
            }
        }
    }

    pub async fn get_live_categories(&self) -> Vec<Value> {
        let url = format!(
            "{}/player_api.php?username={}&password={}&action=get_live_categories",
            self.server, self.username, self.password
        );
        match self.get_vec_url(&url).await {
            Ok(r) => r.clone(),
            _ => std::process::exit(1),
        }
    }

    pub async fn get_vod_categories(&self) -> Vec<Value> {
        let url = format!(
            "{}/player_api.php?username={}&password={}&action=get_vod_categories",
            self.server, self.username, self.password
        );
        match self.get_vec_url(&url).await {
            Ok(r) => r.clone(),
            _ => std::process::exit(1),
        }
    }

    pub async fn get_series_categories(&self) -> Vec<Value> {
        let url = format!(
            "{}/player_api.php?username={}&password={}&action=get_series_categories",
            self.server, self.username, self.password
        );
        match self.get_vec_url(&url).await {
            Ok(r) => r.clone(),
            _ => std::process::exit(1),
        }
    }

    pub async fn get_live_streams(&self, id: Option<u32>) -> Vec<Value> {
        let mut url = format!(
            "{}/player_api.php?username={}&password={}&action=get_live_streams",
            self.server, self.username, self.password
        )
        .to_owned();
        if let Some(i) = id {
            url.push_str(format!("&category_id={i}").as_str());
        };
        match self.get_vec_url(&url).await {
            Ok(r) => r.clone(),
            _ => std::process::exit(1),
        }
    }

    pub async fn get_vod_streams(&self, id: Option<u32>) -> Vec<Value> {
        let mut url = format!(
            "{}/player_api.php?username={}&password={}&action=get_vod_streams",
            self.server, self.username, self.password
        )
        .to_owned();
        if let Some(i) = id {
            url.push_str(format!("&category_id={i}").as_str());
        };
        match self.get_vec_url(&url).await {
            Ok(r) => r.clone(),
            _ => std::process::exit(1),
        }
    }

    pub async fn get_series_streams(&self, id: Option<u32>) -> Vec<Value> {
        let mut url = format!(
            "{}/player_api.php?username={}&password={}&action=get_series",
            self.server, self.username, self.password
        )
        .to_owned();
        if let Some(i) = id {
            url.push_str(format!("&category_id={i}").as_str());
        };
        match self.get_vec_url(&url).await {
            Ok(r) => r.clone(),
            _ => std::process::exit(1),
        }
    }

    pub async fn get_short_epg(&self, id: u32, limit: Option<u32>) -> Vec<Value> {
        let mut url = format!(
            "{}/player_api.php?username={}&password={}&action=get_short_epg&stream_id={id}",
            self.server, self.username, self.password
        )
        .to_owned();
        if let Some(l) = limit {
            url.push_str(format!("&limit={l}").as_str());
        };
        match self.get_vec_url(&url).await {
            Ok(r) => r.clone(),
            _ => std::process::exit(1),
        }
    }
}

pub fn new<'a>(server: &'a String, username: &'a String, password: &'a String) -> Server<'a> {
    Server {
        server,
        username,
        password,
    }
}
