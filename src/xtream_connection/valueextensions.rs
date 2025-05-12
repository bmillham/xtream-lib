use chrono::{DateTime, NaiveDateTime};
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
    fn expires(&self) -> NaiveDateTime;
    fn created(&self) -> NaiveDateTime;
    fn max_connections(&self) -> i32;
    fn is_trial(&self) -> bool;
    fn status(&self) -> String;
    fn active_cons(&self) -> i32;
    fn get_ext(&self) -> String;
    fn get_num(&self) -> i32;
    fn to_date(&self) -> NaiveDateTime;
    fn to_bool(&self) -> bool;
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
    fn expires(&self) -> NaiveDateTime {
        self["user_info"]["exp_date"].to_date()
    }
    fn created(&self) -> NaiveDateTime {
        self["user_info"]["created_at"].to_date()
    }
    fn to_date(&self) -> NaiveDateTime {
        let added_ts = match self.as_str() {
            Some(s) => s.parse().unwrap(),
            _ => self.as_i64().unwrap_or_default(),
        };
        DateTime::from_timestamp(added_ts, 0)
            .unwrap_or_default()
            .naive_utc()
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
        self["user_info"]["is_trial"].to_bool()
    }
    fn to_bool(&self) -> bool {
        match self.is_boolean() {
            true => self.as_bool().unwrap(),
            false => matches!(self.as_str(), Some("1")),
        }
    }
}
