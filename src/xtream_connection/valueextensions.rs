use chrono::DateTime;
use serde_json::Value;

pub trait ValueExtensions {
    fn get_name(&self) -> String;
    fn get_category_name(&self) -> &str;
    fn get_category_id(&self) -> &str;
    fn get_stream_id(&self) -> String;
    fn expires(&self) -> String;
    fn created(&self) -> String;
    fn max_connections(&self) -> i64;
    fn is_trial(&self) -> bool;
    fn status(&self) -> &str;
    fn active_cons(&self) -> &str;
    fn get_ext(&self) -> String;
    fn get_num(&self) -> String;
}

impl ValueExtensions for Value {
    fn get_name(&self) -> String {
        self["name"].as_str().unwrap_or_default().to_string()
    }
    fn get_category_name(&self) -> &str {
        self["category_name"].as_str().unwrap_or_default()
    }
    fn get_category_id(&self) -> &str {
        self["category_id"].as_str().unwrap_or("-1")
    }
    fn get_stream_id(&self) -> String {
        match self["stream_id"].is_string() {
            true => self["stream_id"].as_str().unwrap().to_string(),
            false => self["stream_id"].as_i64().unwrap().to_string(),
        }
    }
    fn get_ext(&self) -> String {
        let x = self["container_extension"].as_str().unwrap_or_default();
        match x.is_empty() {
            true => "".to_string(),
            false => format!(".{x}"),
        }
    }
    fn get_num(&self) -> String {
        match self["num"].is_string() {
            true => self["num"].as_str().unwrap().to_string(),
            false => self["num"].as_i64().unwrap().to_string(),
        }
    }
    fn expires(&self) -> String {
        let exp_ts = match self["user_info"]["exp_date"].as_str() {
            Some(s) => s.parse().unwrap(),
            _ => self["user_info"]["exp_date"].as_i64().unwrap_or_default(),
        };
        DateTime::from_timestamp(exp_ts, 0)
            .unwrap_or_default()
            .to_string()
    }
    fn created(&self) -> String {
        let created_ts = match self["user_info"]["created_at"].as_str() {
            Some(s) => s.parse().unwrap(),
            _ => self["user_info"]["created_at"].as_i64().unwrap_or_default(),
        };
        DateTime::from_timestamp(created_ts, 0)
            .unwrap_or_default()
            .to_string()
    }
    fn max_connections(&self) -> i64 {
        match self["user_info"]["max_connections"].as_str() {
            Some(s) => s.parse().unwrap(),
            _ => self["user_info"]["max_connections"]
                .as_i64()
                .unwrap_or_default(),
        }
    }
    fn status(&self) -> &str {
        self["user_info"]["status"].as_str().unwrap_or_default()
    }
    fn active_cons(&self) -> &str {
        self["user_info"]["active_cons"]
            .as_str()
            .unwrap_or_default()
    }
    fn is_trial(&self) -> bool {
        match self["user_info"]["is_trial"].is_boolean() {
            true => self["user_info"]["is_trial"].as_bool().unwrap(),
            false => matches!(self["user_info"]["is_trial"].as_str(), Some("1")),
        }
    }
}
