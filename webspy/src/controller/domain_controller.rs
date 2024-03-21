use actix_web::{HttpResponse, post, Responder, web};
use crate::data_transfer_object::new_domain::NewDomain;
use crate::service::AppState;
use crate::service::domain_service::save_domain;

#[post("/domain/new")]
pub async fn new_domain(new_domain: web::Json<NewDomain>, db: web::Data<AppState>) -> impl Responder {
    match save_domain(&new_domain, db).await{
        Ok(a) => {
            println!("{:?}", a);
            HttpResponse::Ok().body(format!("New domain added: {} \t {} \t at {}", a.name, a.url, a.timestamp))
        }
        Err(a) => {
            println!("{}", a);
            HttpResponse::BadRequest().body(format!("OOOOOPS! There was an error: {}", a.to_string()))
        }
    }
}