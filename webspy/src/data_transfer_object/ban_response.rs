use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BanResponse{
    pub is_blocked:bool,
    pub message:String,
    pub expire:String,
    //todo: add more?
}