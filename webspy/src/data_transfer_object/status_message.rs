use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StatusMessage{
    pub success: bool,
    pub message: String
}