use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, post, Responder, web};
use sqlx::types::chrono::Local;
use crate::data_transfer_object::ban_response::BanResponse;
use crate::data_transfer_object::report::Report;
use crate::service::AppState;
use crate::service::ban_service::ban_check;
use crate::service::report_service::save_request;

#[post("/report")]
pub async fn report_request(report: web::Json<Report>, db: web::Data<AppState>) -> impl Responder {
    match save_request(&report, &db).await{
        Ok(a) => {
            println!("{:?}", a);
        }
        Err(a) => {
            println!("{}", a);
        }
    }

    match ban_check(&report.ip, &db).await{
        None => {
            // ban does not exist
            let best_response = BanResponse{
                is_blocked: false,
                message: "User is not banned.".to_string(),
                expire: Local::now()
            };

            let ser_response = serde_json::to_string(&best_response).unwrap();
            HttpResponse::Ok().insert_header(ContentType::json()).body(ser_response)
        }
        Some(ban) => {
            // ban exists
            if ban.expire > Local::now(){
                // user is banned
                let bad_response = BanResponse{
                    is_blocked: true,
                    message: ban.reason,
                    expire: ban.expire
                };

                let ser_response = serde_json::to_string(&bad_response).unwrap();
                HttpResponse::Ok().insert_header(ContentType::json()).body(ser_response)
            }else{
                // user has been banned in the past, but is not anymore
                let ok_response = BanResponse{
                    is_blocked: false,
                    message: format!("User is not banned anymore. Old reason: {}", ban.reason),
                    expire: ban.expire
                };

                let ser_response = serde_json::to_string(&ok_response).unwrap();
                HttpResponse::Ok().insert_header(ContentType::json()).body(ser_response)
            }
        }
    }
}