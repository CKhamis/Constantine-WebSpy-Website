use crate::data_transfer_object::new_domain::NewDomain;
use crate::service::domain_service::{get_domains, save_domain};
use crate::service::AppState;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/api/domain/new")]
#[tracing::instrument]
pub async fn new_domain(
    new_domain: web::Json<NewDomain>,
    db: web::Data<AppState>,
) -> impl Responder {
    match save_domain(&new_domain, db).await {
        Ok(a) => {
            println!("{:?}", a);
            HttpResponse::Ok().body(format!(
                "New domain added: {} \t {} \t at {}",
                a.name, a.domain, a.timestamp
            ))
        }
        Err(a) => {
            println!("{}", a);
            HttpResponse::BadRequest().body(format!("OOOOOPS! There was an error: {}", a))
        }
    }
}

#[get("/api/domain/all")]
#[tracing::instrument]
pub async fn all_domains(db: web::Data<AppState>) -> impl Responder {
    let current_domains = get_domains(db).await;
    match serde_json::to_string(&current_domains) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("All domains could not be serialized"),
    }
}
