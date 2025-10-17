use serde_derive::{Serialize, Deserialize};
use super::account::Account;

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub user_info: Account,
}
