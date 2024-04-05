use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewDomain {
    pub name: String,
    pub url: String,
}
