use sea_orm::DatabaseConnection;
pub mod report_service;
pub mod domain_service;
pub mod ban_service;

#[derive(Debug, Clone)]
pub struct AppState {
    //templates: tera::Tera,
    pub conn: DatabaseConnection,
}