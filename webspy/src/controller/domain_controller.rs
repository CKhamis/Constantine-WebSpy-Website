use crate::data_transfer_object::new_domain::NewDomain;
use crate::service::domain_service::{get_domains, save_domain};
use crate::service::AppState;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::{error, info};

#[post("/api/domain/new")]
#[tracing::instrument]
pub async fn new_domain(
    new_domain: web::Json<NewDomain>,
    db: web::Data<AppState>,
) -> impl Responder {
    info!("Handling new domain API request");
    match save_domain(&new_domain, db).await {
        Ok(a) => {
            info!("Successfully saved domain to database: {:?}", a);
            HttpResponse::Ok().body(format!(
                "New domain added: {} \t {} \t at {}",
                a.name, a.domain, a.timestamp
            ))
        }
        Err(a) => {
            error!("Error adding new domain to database: {}", a);
            HttpResponse::BadRequest().body(format!("OOOOOPS! There was an error: {}", a))
        }
    }
}

#[get("/api/domain/all")]
#[tracing::instrument]
pub async fn all_domains(db: web::Data<AppState>) -> impl Responder {
    info!("Handling all domain API request");
    let current_domains = get_domains(db).await;
    match serde_json::to_string(&current_domains) {
        Ok(response) => {
            info!("Successfully serialized all domains to json: {}", response);
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error deserializing all domains to json: {}", e);
            HttpResponse::BadRequest().body("All domains could not be serialized")
        }
    }
}
