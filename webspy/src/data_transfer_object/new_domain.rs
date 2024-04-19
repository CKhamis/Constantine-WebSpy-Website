use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewDomain {
    pub name: String,
    pub url: String,
}
