use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse, post, Responder, web};
use sqlx::types::chrono::Local;
use crate::data_transfer_object::ban_response::BanResponse;
use crate::data_transfer_object::new_user::NewUser;
use crate::data_transfer_object::report::Report;
use crate::service::AppState;
use crate::service::user_service::{active_users, new_user, user_check};
use crate::service::report_service::{find_by_user, save_request, verify_domain};

#[post("/report")]
pub async fn report_request(report: web::Json<Report>, app_state: web::Data<AppState>) -> impl Responder {
    println!("/report received request");

    // Check if domain is in database
    if !verify_domain(&report.domain_id, &app_state.conn).await{
        print!("Received invalid API key for a request");
        return HttpResponse::BadRequest().body("Invalid report received. Rejected")
    }

    println!("User exists in database: {:?}", user_check(&report.ip, &app_state).await);

    // Check if user already exists in database and create if it doesn't
    match user_check(&report.ip, &app_state).await.map_or(None, |a| {a}){
        None => {
            // user does not exist, create it
            let user = NewUser{
                expire: None,
                ip: report.ip.clone(),
                message: None
            };

            // Save user
            match new_user(user, &app_state).await {
                Ok(_) => {
                    let best_response = BanResponse{
                        is_blocked: false,
                        message: "User is not banned.".to_string(),
                        expire: Local::now()
                    };

                    // Save request
                    match save_request(&report, false, &app_state).await{
                        Ok(a) => {
                            //println!("{:?}", a);
                        }
                        Err(a) => {
                            println!("{}", a);
                        }
                    }

                    // Return response
                    let ser_response = serde_json::to_string(&best_response).unwrap();
                    HttpResponse::Ok().insert_header(ContentType::json()).body(ser_response)
                }
                Err(e) => {
                    dbg!(&e);
                    return HttpResponse::BadRequest().body(format!("OOOOOPS! There was an error :( {}", e.to_string()))
                }
            }
        }
        Some(user) => {
            // user exists
            if let Some(expire_date) = user.expire{
                if expire_date > Local::now(){
                    // user is banned
                    let bad_response = BanResponse{
                        is_blocked: true,
                        message: user.reason.unwrap_or("No reason given".to_string()),
                        expire: expire_date
                    };

                    // Save request
                    match save_request(&report, true, &app_state).await{
                        Ok(a) => {
                            //println!("{:?}", a);
                        }
                        Err(a) => {
                            println!("{}", a);
                        }
                    }

                    let ser_response = serde_json::to_string(&bad_response).unwrap();
                    HttpResponse::Ok().insert_header(ContentType::json()).body(ser_response)
                }else{
                    // user was seen before, but is not banned
                    let ok_response = BanResponse{
                        is_blocked: false,
                        message: format!("User is not banned anymore. Old reason: {}", user.reason.unwrap_or("No reason given".to_string())),
                        expire: expire_date
                    };

                    // save request
                    match save_request(&report, false, &app_state).await{
                        Ok(a) => {
                            //println!("{:?}", a);
                        }
                        Err(a) => {
                            println!("{}", a);
                        }
                    }

                    let ser_response = serde_json::to_string(&ok_response).unwrap();
                    HttpResponse::Ok().insert_header(ContentType::json()).body(ser_response)
                }
            }else{
                // User was never banned
                let ok_response = BanResponse{
                    is_blocked: false,
                    message: "User was not banned".to_string(),
                    expire: Local::now()
                };

                match save_request(&report, false, &app_state).await{
                    Ok(a) => {
                        //println!("{:?}", a);
                    }
                    Err(a) => {
                        println!("{}", a);
                    }
                }

                let ser_response = serde_json::to_string(&ok_response).unwrap();
                HttpResponse::Ok().insert_header(ContentType::json()).body(ser_response)
            }
        }
    }
}

#[get("/api/report/user/{ip}")]
pub async fn get_report_by_user(ip_address: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder{
    match serde_json::to_string(&find_by_user(&ip_address, &app_state.conn).await){
        Ok(response) => {HttpResponse::Ok().insert_header(ContentType::json()).body(response)}
        Err(_) => {HttpResponse::BadRequest().body("There was an error serializing")}
    }
}