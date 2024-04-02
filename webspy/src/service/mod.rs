use handlebars::Handlebars;
use sea_orm::DatabaseConnection;
pub mod report_service;
pub mod domain_service;
pub mod user_service;
pub mod analytics_service;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}