use crate::service::analytics_service::{
    daily_activity, daily_activity_by_user, daily_activity_by_user_by_domain, domain_activity,
    domain_activity_by_user, endpoint_frequency, endpoint_frequency_by_user,
};
use crate::service::AppState;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder};
use tracing::{error, info};

#[get("/api/analytics/daily-requests")]
#[tracing::instrument]
pub async fn daily_requests(db: web::Data<AppState>) -> impl Responder {
    info!("Handling API request for daily-requests");
    let data = daily_activity(&db).await;
    match serde_json::to_string(&data) {
        Ok(response) => {
            info!(
                "Successfully serialized daily-requests to json: {}",
                response
            );
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error serializing daily-requests data to json: {}", e);
            HttpResponse::BadRequest().body("data could not be fetched")
        }
    }
}

#[get("/api/analytics/domain-requests")]
#[tracing::instrument]
pub async fn domain_requests(db: web::Data<AppState>) -> impl Responder {
    info!("Handling API request for domain-requests");
    match serde_json::to_string(&domain_activity(&db).await) {
        Ok(response) => {
            info!(
                "Successfully serialized domain-requests to json: {}",
                response
            );
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error serializing domain-requests data to json: {}", e);
            HttpResponse::BadRequest().body("data could not be fetched")
        }
    }
}

#[get("/api/analytics/endpoint-requests")]
#[tracing::instrument]
pub async fn get_endpoint_frequency(db: web::Data<AppState>) -> impl Responder {
    info!("Handling API request for endpoint-requests");
    match serde_json::to_string(&endpoint_frequency(&db).await) {
        Ok(response) => {
            info!(
                "Successfully serialized endpoint frequency to json: {}",
                response
            );
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error serializing endpoint frequency data to json: {}", e);
            HttpResponse::BadRequest().body("data could not be fetched")
        }
    }
}

#[get("/api/analytics/daily-requests/{ip_address}")]
#[tracing::instrument]
pub async fn daily_requests_by_user(
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Handling API request for daily-requests for user: {}",
        ip_address
    );
    match serde_json::to_string(&daily_activity_by_user(ip_address.trim(), &db).await) {
        Ok(response) => {
            info!(
                "Successfully serialized daily-requests for a user to json: {}",
                response
            );
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!("Error serializing daily-requests data to json: {}", e);
            HttpResponse::BadRequest().body("data could not be fetched")
        }
    }
}

#[get("/api/analytics/daily-requests-by-domain/{ip_address}")]
#[tracing::instrument]
pub async fn daily_requests_by_user_by_domain(
    // TODO(costi): update this endpoint to be more consistent e.g. missing 'by user'
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Handling API request for daily-requests-by-domain for: {}",
        ip_address
    );
    match serde_json::to_string(&daily_activity_by_user_by_domain(ip_address.trim(), &db).await) {
        Ok(response) => {
            info!(
                "Successfully serialized daily-requests-by-domain to json: {}",
                response
            );
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!(
                "Error serializing daily-requests-by-domain data to json: {}",
                e
            );
            HttpResponse::BadRequest().body("data could not be fetched")
        }
    }
}

#[get("/api/analytics/domain-requests/{ip_address}")]
#[tracing::instrument]
pub async fn domain_requests_by_user(
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Handling API request for domain-requests by user for: {}",
        ip_address
    );
    match serde_json::to_string(&domain_activity_by_user(&ip_address, &db).await) {
        Ok(response) => {
            info!(
                "Successfully serialized domain-requests by user to json: {}",
                response
            );
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!(
                "Error serializing domain-requests by user data to json: {}",
                e
            );
            HttpResponse::BadRequest().body("data could not be fetched")
        }
    }
}

#[get("/api/analytics/endpoint-requests/{ip_address}")]
#[tracing::instrument]
pub async fn get_endpoint_frequency_by_user(
    ip_address: web::Path<String>,
    db: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Handling API request for endpoint-requests by user for: {}",
        ip_address
    );
    match serde_json::to_string(&endpoint_frequency_by_user(&ip_address, &db).await) {
        Ok(response) => {
            info!(
                "Successfully serialized endpoint-requests by user to json: {}",
                response
            );
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(e) => {
            error!(
                "Error serializing endpoint-requests by user data to json: {}",
                e
            );
            HttpResponse::BadRequest().body("data could not be fetched")
        }
    }
}
