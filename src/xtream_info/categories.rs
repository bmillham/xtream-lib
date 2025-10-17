use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub category_id: String,
    pub category_name: String,
    parent_id: u32,
}

pub trait CategoryExtensions {
    fn get_category_id(&self) -> u32;
}

impl CategoryExtensions for Category {
    fn get_category_id(&self) -> u32 {
        self.category_id.parse::<u32>().unwrap()
    }
}

