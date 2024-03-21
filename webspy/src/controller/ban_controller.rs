use actix_web::{get, HttpResponse, post, Responder, web};
use sea_orm::DbErr;
use crate::data_transfer_object::new_ban::NewBan;
use crate::model::ban::Model;
use crate::service::{AppState, ban_service};
use crate::service::ban_service::save_ban;
use crate::service::domain_service::get_domains;

#[get("/ban/all")]
pub async  fn all_bans(db: web::Data<AppState>) -> impl Responder{
    let current_domains = get_domains(db).await;
    match serde_json::to_string(&current_domains){
        Ok(response) => {HttpResponse::Ok().body(response)}
        Err(_) => {HttpResponse::BadRequest().body("All domains could not be serialized")}
    }
}

#[post("/ban/new")]
pub async  fn new_ban(new_ban_request: web::Json<NewBan>, db: web::Data<AppState>) -> impl Responder{
    match save_ban(&new_ban_request, &db).await{
        Ok(obj) => {HttpResponse::Ok().body(format!("IP address: {} was banned.", obj.ip))}
        Err(err) => {HttpResponse::BadRequest().body(format!("OOOPS >:) {}", err))}
    }
}