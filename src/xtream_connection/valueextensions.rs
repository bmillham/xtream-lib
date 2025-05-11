use chrono::DateTime;
use serde_json::Value;
use std::str::FromStr;

pub trait ValueExtensions {
    fn to_i32(&self) -> i32;
    fn t_string(&self) -> String;
    fn get_name(&self) -> String;
    fn get_category_name(&self) -> String;

    fn get_category_id(&self) -> i32;
    fn get_parent_id(&self) -> i32;
    fn get_stream_id(&self) -> i32;
    fn expires(&self) -> String;
    fn created(&self) -> String;
    fn max_connections(&self) -> i32;
    fn is_trial(&self) -> bool;
    fn status(&self) -> String;
    fn active_cons(&self) -> i32;
    fn get_ext(&self) -> String;
    fn get_num(&self) -> i32;
}

impl ValueExtensions for Value {
    fn to_i32(&self) -> i32 {
        match self.as_str() {
            Some(s) => i32::from_str(s).unwrap_or(-1),
            _ => self.as_i64().unwrap_or(-1) as i32,
        }
    }
    fn t_string(&self) -> String {
        self.as_str().unwrap_or_default().to_string()
    }
    fn get_name(&self) -> String {
        self["name"].t_string()
    }
    fn get_category_name(&self) -> String {
        self["category_name"].t_string()
    }

    fn get_category_id(&self) -> i32 {
        self["category_id"].to_i32()
    }
    fn get_parent_id(&self) -> i32 {
        self["parent_id"].to_i32()
    }
    fn get_stream_id(&self) -> i32 {
        self["stream_id"].to_i32()
    }
    fn get_ext(&self) -> String {
        let x = self["container_extension"].t_string();
        match x.is_empty() {
            true => "".to_string(),
            false => format!(".{x}"),
        }
    }
    fn get_num(&self) -> i32 {
        self["num"].to_i32()
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
    fn max_connections(&self) -> i32 {
        self["user_info"]["max_connections"].to_i32()
    }
    fn status(&self) -> String {
        self["user_info"]["status"].t_string()
    }
    fn active_cons(&self) -> i32 {
        self["user_info"]["active_cons"].to_i32()
    }
    fn is_trial(&self) -> bool {
        match self["user_info"]["is_trial"].is_boolean() {
            true => self["user_info"]["is_trial"].as_bool().unwrap(),
            false => matches!(self["user_info"]["is_trial"].as_str(), Some("1")),
        }
    }
}
