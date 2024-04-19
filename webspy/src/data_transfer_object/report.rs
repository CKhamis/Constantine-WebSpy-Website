use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
//#[serde(default)]
pub struct Report {
    // Authentication
    pub domain_id: String,

    // Client info
    pub ip: String,
    pub client_host: Option<String>,
    pub client_port: Option<String>,
    pub client_user: Option<String>,
    pub client_locale: Option<String>,

    // Session info
    pub session: Option<String>,
    pub cookies: Option<String>,

    // Request info
    pub request_uri: Option<String>,
    pub request_url: String,
    pub request_method: String,
    pub request_header: String,
    pub request_protocol: String,
    pub request_scheme: String,
    pub user_agent: String,
}
