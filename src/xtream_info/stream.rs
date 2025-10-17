use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    pub added: String,
    pub category_id: String,
    pub category_ids: Vec<u32>,
    pub custom_sid: Option<u32>,
    pub direct_source: String,
    pub epg_channel_id: String,
    pub is_adult: u8,
    pub name: String,
    pub num: i32,
    pub stream_icon: String,
    pub stream_id: u32,
    pub stream_type: String,
    pub tv_archive: u8,
    pub tv_archive_duration: u32,
}
