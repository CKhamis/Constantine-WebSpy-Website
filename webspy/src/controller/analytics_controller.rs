use crate::service::analytics_service::{
    daily_activity, daily_activity_by_ip, daily_activity_by_ip_by_domain,
    daily_blocked_activity, domain_activity, domain_activity_by_ip, endpoint_frequency,
    endpoint_frequency_by_ip, request_total, unique_ips_per_domain,
};
use crate::service::AppState;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/analytics/daily-requests")]
pub async fn daily_requests(db: web::Data<AppState>) -> impl Responder {
    let data = daily_activity(&db).await;
    match serde_json::to_string(&data) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/daily-blocked-requests")]
pub async fn daily_blocked_requests(db: web::Data<AppState>) -> impl Responder {
    let data = daily_blocked_activity(&db).await;
    match serde_json::to_string(&data) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/domain-requests")]
pub async fn domain_requests(db: web::Data<AppState>) -> impl Responder {
    match serde_json::to_string(&domain_activity(&db).await) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/endpoint-requests")]
pub async fn get_endpoint_frequency(db: web::Data<AppState>) -> impl Responder {
    match serde_json::to_string(&endpoint_frequency(&db).await) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/request-total")]
pub async fn get_endpoint_frequency_total(db: web::Data<AppState>) -> impl Responder {
    match serde_json::to_string(&request_total(&db).await) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/daily-requests/{ip_address}")]
pub async fn daily_requests_by_ip(
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    match serde_json::to_string(&daily_activity_by_ip(ip_address.trim(), &db).await) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/daily-requests-by-domain/{ip_address}")]
pub async fn daily_requests_by_ip_by_domain(
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    match serde_json::to_string(&daily_activity_by_ip_by_domain(ip_address.trim(), &db).await) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/domain-requests/{ip_address}")]
pub async fn domain_requests_by_ip(
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    match serde_json::to_string(&domain_activity_by_ip(&ip_address, &db).await) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

#[get("/api/analytics/endpoint-requests/{ip_address}")]
pub async fn get_endpoint_frequency_by_ip(
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    match serde_json::to_string(&endpoint_frequency_by_ip(&ip_address, &db).await) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}

// Domain Information
#[get("/api/analytics/unique-ips-all-domains")]
pub async fn unique_visitors(db: web::Data<AppState>) -> impl Responder {
    let data = unique_ips_per_domain(&db).await;
    match serde_json::to_string(&data) {
        Ok(response) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(response),
        Err(_) => HttpResponse::BadRequest().body("data could not be fetched"),
    }
}
