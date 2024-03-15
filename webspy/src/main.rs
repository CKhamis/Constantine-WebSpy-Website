use webspy::service::AppState;
use std::env;
use actix_web::{App, HttpServer, web};
use sea_orm::{Database, DatabaseConnection};
use webspy::controller::controller_prelude::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo: make env vars to work
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // let host = env::var("HOST").expect("HOST is not set in .env file");
    // let port = env::var("PORT").expect("PORT is not set in .env file");
    // let server_url = format!("{host}:{port}");

    let connection = Database::connect("mysql://root:1234@localhost/web_spy").await.unwrap();
    let app_state = AppState{ conn: connection };

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(app_state.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}