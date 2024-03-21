use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewBan{
    pub ip:String,
    pub message:String,
    pub expire:DateTime<Local>,
}