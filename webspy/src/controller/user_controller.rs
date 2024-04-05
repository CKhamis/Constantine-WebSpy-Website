use crate::service::user_service::{active_users, all_users, banned_users};
use crate::service::AppState;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder};
use tracing::{error, info};

#[get("/api/user/all")]
#[tracing::instrument]
pub async fn get_all_users(db: web::Data<AppState>) -> impl Responder {
    info!("Handling get_all_users API request");
    let all_users = all_users(&db).await;
    match serde_json::to_string(&all_users) {
        Ok(response) => {
            info!("Successfully serialized user list");
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error serializing user list: {}", e);
            HttpResponse::BadRequest().body("There was an error serializing user list")
        }
    }
}

#[get("/api/user/banned")]
#[tracing::instrument]
pub async fn get_banned_users(db: web::Data<AppState>) -> impl Responder {
    info!("Handling banned users API request");
    match serde_json::to_string(&banned_users(&db).await) {
        Ok(response) => {
            info!("Successfully serialized banned user list");
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error serializing list of banned users: {}", e);
            HttpResponse::BadRequest().body("There was an error serializing user list")
        }
    }
}

#[get("/api/user/active")]
#[tracing::instrument]
pub async fn get_all_users_by_activity(db: web::Data<AppState>) -> impl Responder {
    info!("Getting all users by last seen activity");
    match serde_json::to_string(&active_users(&db).await) {
        Ok(response) => {
            info!("Successfully serialized active_users");
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error serializing acive users: {}", e);
            HttpResponse::BadRequest().body("There was an error serializing user list")
        }
    }
}
