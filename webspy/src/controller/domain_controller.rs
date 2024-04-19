use crate::data_transfer_object::new_domain::NewDomain;
use crate::data_transfer_object::status_message::StatusMessage;
use crate::service::domain_service::{get_domains, save_domain};
use crate::service::AppState;
use actix_web::error::UrlencodedError::Serialize;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::ColumnType::Json;
use serde_json::json;

#[post("/api/domain/new")]
pub async fn new_domain(
    new_domain: web::Json<NewDomain>,
    db: web::Data<AppState>,
) -> impl Responder {
    match save_domain(&new_domain, db).await {
        Ok(a) => {
            let response = StatusMessage {
                success: true,
                message: format!("Domain has been added: {}", a.name),
            };
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(json!(response).to_string()) //todo: (cory) check if this is good practice
        }
        Err(a) => {
            println!("{}", a);
            let response = StatusMessage {
                success: false,
                message: a.to_string(),
            };
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(json!(response).to_string())
        }
    }
}

#[get("/api/domain/all")]
pub async fn all_domains(db: web::Data<AppState>) -> impl Responder {
    let current_domains = get_domains(db).await;
    match serde_json::to_string(&current_domains) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("All domains could not be serialized"),
    }
}
