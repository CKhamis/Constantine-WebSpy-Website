use actix_web::web;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use sqlx::types::chrono::Local;
use uuid::Uuid;
use crate::data_transfer_object::report::Report;
use crate::model::request;
use crate::model::request::{Model as Log, Model};
use crate::service::AppState;

pub async fn find_all(conn: &DatabaseConnection){
    let logs: Vec<Model> = request::Entity::find().all(conn).await.unwrap();
    println!("{:?}", logs);
}

pub async fn save_request(report: &web::Json<Report>, db: web::Data<AppState>) -> Result<Model, DbErr> {
    let incoming_request = crate::model::request::ActiveModel{
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
        domain_id: ActiveValue::Set(Uuid::new_v4()),
    };
    incoming_request.insert(&db.conn).await
}