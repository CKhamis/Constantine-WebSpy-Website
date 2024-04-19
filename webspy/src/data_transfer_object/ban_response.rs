use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BanResponse {
    pub is_blocked: bool,
    pub message: String,
    pub expire: DateTime<Local>,
    //todo: add more?
}
