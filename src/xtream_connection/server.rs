use reqwest;
use serde_json::Value;
use crate::xtream_info::account::Account;
use crate::xtream_info::categories::Category;
use crate::xtream_info::stream::Stream;
use crate::xtream_info::user_info::UserInfo;

#[derive(Debug)]
pub struct Server<'a> {
    server: &'a str,
    username: &'a str,
    password: &'a str,
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
                Err(e)
            }
        }
    }

    async fn get_vec_url<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<Vec<T>, reqwest::Error> {
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
    pub async fn get_account_info(&self) -> Result<Account, reqwest::Error> {
        let url = format!(
            "{}/player_api.php?username={}&password={}",
            self.server, self.username, self.password
        );
        match self.get_url::<UserInfo>(&url).await {
            Ok(r) => Ok(r.user_info),
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn get_live_categories(&self) -> Result<Vec<Category>, reqwest::Error> {
        let url = format!(
            "{}/player_api.php?username={}&password={}&action=get_live_categories",
            self.server, self.username, self.password
        );
        //match self.get_vec_url(&url).await {
        match self.get_url::<Vec<Category>>(&url).await {
            Ok(r) => {
                Ok(r)
            },
            Err(e) => {
                Err(e)
            },
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
            Err(e) => {
                println!("Error {e:?}");
                std::process::exit(1)
            },
        }
    }

    pub async fn get_live_streams(&self, id: Option<u32>) -> Result<Vec<Stream>, reqwest::Error> {
        let mut url = format!(
            "{}/player_api.php?username={}&password={}&action=get_live_streams",
            self.server, self.username, self.password
        )
        .to_owned();
        if let Some(i) = id {
            url.push_str(format!("&category_id={i}").as_str());
        };
        //match self.get_vec_url(&url).await {
        match self.get_url::<Vec<Stream>>(&url).await {
            Ok(r) => {
                Ok(r)
            },
            Err(e) => {
                Err(e)
            },
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
