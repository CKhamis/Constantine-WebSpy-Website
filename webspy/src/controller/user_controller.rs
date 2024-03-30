use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use sea_orm::DbErr;
use crate::data_transfer_object::new_user::NewUser;
use crate::model::user::Model;
use crate::service::{AppState, user_service};
use crate::service::user_service::{all_users, new_user};
use crate::service::domain_service::get_domains;

// #[get("/api/ban/all")]
// pub async  fn all_bans(db: web::Data<AppState>) -> impl Responder{
//     let current_bans = get_bans(&db).await;
//     match serde_json::to_string(&current_bans){
//         Ok(response) => {HttpResponse::Ok().insert_header(ContentType::json()).body(response)}
//         Err(_) => {HttpResponse::BadRequest().body("All domains could not be serialized")}
//     }
// }

// #[post("/api/ban/new")]
// pub async  fn new_ban(new_ban_request: web::Json<NewUser>, db: web::Data<AppState>) -> impl Responder{
//     // match new_user(&new_ban_request, &db).await{
//     //     Ok(obj) => {HttpResponse::Ok().body(format!("IP address: {} was banned.", obj.ip))}
//     //     Err(err) => {HttpResponse::BadRequest().body(format!("OOOPS >:) {}", err))}
//     // }
//     todo!()
// }