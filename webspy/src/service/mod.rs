use handlebars::Handlebars;
use sea_orm::DatabaseConnection;
pub mod report_service;
pub mod domain_service;
pub mod ban_service;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}