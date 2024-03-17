use sea_orm::DatabaseConnection;
pub mod request_service;

#[derive(Debug, Clone)]
pub struct AppState {
    //templates: tera::Tera,
    pub conn: DatabaseConnection,
}