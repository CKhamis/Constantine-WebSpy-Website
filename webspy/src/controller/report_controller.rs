use actix_web::{HttpResponse, post, Responder, web};
use handlebars::Handlebars;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr};
use sea_orm::sea_query::ColumnSpec::Default;
use sqlx::types::chrono::Local;
use crate::data_transfer_object::report::Report;
use crate::model::request::Model;
use crate::service::AppState;

//#[post("/report")]
pub async fn report_request(report: web::Json<Report>, db: web::Data<AppState>) -> impl Responder {
    let m = crate::model::request::ActiveModel{
        id: ActiveValue::NotSet,
        ip: ActiveValue::Set(report.ip.to_string()),
        client_host: report.client_host.clone().map_or(ActiveValue::Set("".to_string()), |a|ActiveValue::Set(a)),
        client_port: report.client_port.clone().map_or(ActiveValue::Set("".to_string()), |a|ActiveValue::Set(a)),
        client_user: report.client_user.clone().map_or(ActiveValue::Set("".to_string()), |a|ActiveValue::Set(a)),
        client_locale: report.client_locale.clone().map_or(ActiveValue::Set("".to_string()), |a|ActiveValue::Set(a)),
        session: report.session.clone().map_or(ActiveValue::Set("".to_string()), |a|ActiveValue::Set(a)),
        cookies: report.cookies.clone().map_or(ActiveValue::Set("".to_string()), |a|ActiveValue::Set(a)),
        request_uri: report.request_uri.clone().map_or(ActiveValue::Set("".to_string()), |a|ActiveValue::Set(a)),
        request_url: ActiveValue::Set(report.request_url.to_string()),
        request_method: ActiveValue::Set(report.request_method.to_string()),
        request_header: ActiveValue::Set(report.request_header.to_string()),
        request_protocol: ActiveValue::Set(report.request_protocol.to_string()),
        request_scheme: ActiveValue::Set(report.request_scheme.to_string()),
        timestamp: ActiveValue::Set(Local::now()),
    };
    match m.insert(&db.conn).await{
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