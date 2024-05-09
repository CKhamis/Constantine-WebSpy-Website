use actix_web::{web, App, HttpServer};

use sea_orm::{ConnectionTrait, Database, Schema};
use std::env;

use webspy::controller::analytics_controller::*;
use webspy::controller::controller_prelude::*;
use webspy::controller::domain_controller::{all_domains, new_domain};
use webspy::controller::report_controller::*;
use webspy::controller::report_controller::{
    get_report_by_domain, get_report_by_ip, report_request,
};
use webspy::controller::ip_controller::*;
use webspy::model::{domain, request, ip};

use webspy::service::AppState;
use webspy::util::template_config::template_resources;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo: make env vars to work

    let webserver_bind_address = env::var("WEB_BIND_IP").unwrap_or("0.0.0.0".to_string());
    let webserver_bind_port = env::var("WEB_BIND_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Unable to parse web bind port into a number");

    let db_url = env::var("DB_URL").unwrap_or("/web_spy".to_string());
    let host = env::var("DB_HOST").unwrap_or("localhost".to_string());
    let username = env::var("DB_USERNAME").unwrap_or("root".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or("1234".to_string());

    let server_url = format!("mysql://{username}:{password}@{host}{db_url}");

    #[cfg(debug_assertions)]
    println!("{}", server_url);

    // "mysql://root:1234@localhost/web_spy"
    let connection = Database::connect(server_url).await.unwrap();
    template_resources();

    //Create tables
    let builder = connection.get_database_backend();
    let schema = Schema::new(builder);
    let domain_table = builder.build(&schema.create_table_from_entity(domain::Entity));
    match connection.execute(domain_table).await {
        Ok(_) => {
            println!("Creating new table: domain");
        }
        Err(e) => {
            // Crash program if table could not be created if not exists
            println!("{}", e);
            assert!(
                e.to_string().contains("1050") && e.to_string().contains("already exists"),
                "{:?}",
                e
            );
        }
    }
    let ban_table = builder.build(&schema.create_table_from_entity(ip::Entity));
    match connection.execute(ban_table).await {
        Ok(_) => {
            println!("Creating new table: ban");
        }
        Err(e) => {
            // Crash program if table could not be created if not exists
            assert!(
                e.to_string().contains("1050") && e.to_string().contains("already exists"),
                "{:?}",
                e
            );
        }
    }
    let request_table = builder.build(&schema.create_table_from_entity(request::Entity));
    match connection.execute(request_table).await {
        Ok(_) => {
            println!("Creating new table: request");
        }
        Err(e) => {
            // Crash program if table could not be created if not exists
            assert!(
                e.to_string().contains("1050") && e.to_string().contains("already exists"),
                "{:?}",
                e
            );
        }
    }

    let app_state = AppState { conn: connection };
    println!("//////////// Constantine WebSpy //////////////");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(dashboard)
            .service(echo)
            .service(new_domain)
            .service(all_domains)
            .service(report_request)
            .service(get_banned_ips)
            .service(get_all_ip_by_activity)
            .service(get_all_ips)
            .service(daily_requests)
            .service(daily_requests_by_user)
            .service(domain_requests)
            .service(domain_requests_by_user)
            .service(get_report_by_ip)
            .service(get_report_by_domain)
            .service(get_endpoint_frequency_by_user)
            .service(get_endpoint_frequency)
            .service(daily_requests_by_user_by_domain)
            .service(unique_visitors)
            .service(get_endpoint_frequency_total)
            .service(daily_blocked_requests)
            .service(get_all_reports)
            // .service(new_ban)
            // .service(all_bans)
            .service(actix_files::Files::new(
                "/static",
                "./webspy/resources/static",
            ))
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(app_state.clone()))
    })
    .bind((webserver_bind_address, webserver_bind_port))?
    .run()
    .await
}
