use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use crate::service::{AppState, user_service};
use crate::service::user_service::{active_users, all_users, banned_users};

#[get("/api/user/all")]
pub async fn get_all_users(db: web::Data<AppState>) -> impl Responder{
    let all_users = all_users(&db).await;
    match serde_json::to_string(&all_users){
        Ok(response) => {HttpResponse::Ok().insert_header(ContentType::json()).body(response)}
        Err(_) => {HttpResponse::BadRequest().body("There was an error serializing user list")}
    }
}

#[get("/api/user/banned")]
pub async fn get_banned_users(db: web::Data<AppState>) -> impl Responder{
    match serde_json::to_string(&banned_users(&db).await){
        Ok(response) => {HttpResponse::Ok().insert_header(ContentType::json()).body(response)}
        Err(_) => {HttpResponse::BadRequest().body("There was an error serializing user list")}
    }
}

#[get("/api/user/active")]
pub async fn get_all_users_by_activity(db: web::Data<AppState>) -> impl Responder{
    match serde_json::to_string(&active_users(&db).await){
        Ok(response) => {HttpResponse::Ok().insert_header(ContentType::json()).body(response)}
        Err(_) => {HttpResponse::BadRequest().body("There was an error serializing user list")}
    }
}