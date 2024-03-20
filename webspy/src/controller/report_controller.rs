use actix_web::{HttpResponse, post, Responder, web};
use handlebars::Handlebars;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr};
use sea_orm::sea_query::ColumnSpec::Default;
use sqlx::types::chrono::Local;
use crate::data_transfer_object::report::Report;
use crate::model::request::Model;
use crate::service::AppState;
use crate::service::request_service::save_request;

//#[post("/report")] todo: check if I can uncomment this for consistency
pub async fn report_request(report: web::Json<Report>, db: web::Data<AppState>) -> impl Responder {
    match save_request(&report, db).await{
        Ok(a) => {
            println!("{:?}", a);
        }
        Err(a) => {
            println!("{}", a);
        }
    }


    println!("{:?}", report.request_url);
    HttpResponse::Ok().body("Hey there!")
}