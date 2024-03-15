use sea_orm::DatabaseConnection;
pub mod log_service;

#[derive(Debug, Clone)]
pub struct AppState {
    //templates: tera::Tera,
    pub conn: DatabaseConnection,
}