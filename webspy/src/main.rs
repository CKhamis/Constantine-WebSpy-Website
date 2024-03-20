use webspy::service::AppState;
use std::env;
use actix_web::{App, HttpServer, web};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, ExecResult, Schema};
use webspy::controller::controller_prelude::*;
use webspy::controller::report_controller::report_request;
use webspy::model::{domain, request};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo: make env vars to work
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // let host = env::var("HOST").expect("HOST is not set in .env file");
    // let port = env::var("PORT").expect("PORT is not set in .env file");
    // let server_url = format!("{host}:{port}");

    let connection = Database::connect("mysql://root:1234@localhost/web_spy").await.unwrap();

    //Create tables
    let builder = connection.get_database_backend();
    let schema = Schema::new(builder);
    let domain_table = builder.build(&schema.create_table_from_entity(domain::Entity));
    match connection.execute(domain_table).await{
        Ok(a) => {println!("{:?}", a)}
        Err(e) => {println!("{:?}", e)}
    }
    let request_table = builder.build(&schema.create_table_from_entity(request::Entity));
    match connection.execute(request_table).await{
        Ok(a) => {println!("{:?}", a)}
        Err(e) => {println!("{:?}", e)}
    }

    let app_state = AppState{ conn: connection };
    println!("terce");
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .route("/report", web::post().to(report_request))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(app_state.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}