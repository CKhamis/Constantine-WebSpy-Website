use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct newIp {
    pub ip: String,
    pub message: Option<String>,
    pub expire: Option<DateTime<Local>>,
}
