use webspy::service::AppState;
use std::env;
use std::sync::RwLock;
use actix_web::{App, HttpServer, web};
use handlebars::Handlebars;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, ExecResult, RuntimeErr, Schema};
use webspy::controller::ban_controller::{all_bans, new_ban};
use webspy::controller::controller_prelude::*;
use webspy::controller::domain_controller::{all_domains, new_domain};
use webspy::controller::report_controller::report_request;
use webspy::HANDLEBARS_TEMPLATE;
use webspy::model::{ban, domain, request};
use webspy::util::template_config::template_resources;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo: make env vars to work
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // let host = env::var("HOST").expect("HOST is not set in .env file");
    // let port = env::var("PORT").expect("PORT is not set in .env file");
    // let server_url = format!("{host}:{port}");

    let connection = Database::connect("mysql://root:1234@localhost/web_spy").await.unwrap();
    template_resources();

    //Create tables
    let builder = connection.get_database_backend();
    let schema = Schema::new(builder);
    let domain_table = builder.build(&schema.create_table_from_entity(domain::Entity));
    match connection.execute(domain_table).await{
        Ok(_) => {println!("Creating new table: domain");}
        Err(e) => {
            // Crash program if table could not be created if not exists
            println!("{}", e);
            assert!(e.to_string().contains("1050") && e.to_string().contains("already exists"), "{:?}", e);
        }
    }
    let request_table = builder.build(&schema.create_table_from_entity(request::Entity));
    match connection.execute(request_table).await{
        Ok(_) => {println!("Creating new table: request");}
        Err(e) => {
            // Crash program if table could not be created if not exists
            assert!(e.to_string().contains("1050") && e.to_string().contains("already exists"), "{:?}", e);
        }
    }
    let ban_table = builder.build(&schema.create_table_from_entity(ban::Entity));
    match connection.execute(ban_table).await{
        Ok(_) => {println!("Creating new table: ban");}
        Err(e) => {
            // Crash program if table could not be created if not exists
            assert!(e.to_string().contains("1050") && e.to_string().contains("already exists"), "{:?}", e);
        }
    }

    let app_state = AppState{ conn: connection };
    println!("//////////// Constantine WebSpy //////////////");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(dashboard)
            .service(echo)
            .service(new_domain)
            .service(all_domains)
            .service(report_request)
            .service(new_ban)
            .service(all_bans)
            .service(actix_files::Files::new("/static", "./webspy/resources/static"))
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(app_state.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}