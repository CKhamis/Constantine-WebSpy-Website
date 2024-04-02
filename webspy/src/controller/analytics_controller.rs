use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use crate::service::analytics_service::daily_activity;
use crate::service::AppState;

#[get("/api/analytics/daily-requests")]
pub async fn daily_requests(db: web::Data<AppState>) -> impl Responder{
    let data = daily_activity(&db).await;
    match serde_json::to_string(&data){
        Ok(response) => {HttpResponse::Ok().insert_header(ContentType::json()).body(response)}
        Err(_) => {HttpResponse::BadRequest().body("data could not be fetched")}
    }
}