use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder,
};
use sqlx::types::chrono::Local;
use tracing::{error, info};

use crate::data_transfer_object::report::Report;
use crate::model::request::Model;
use crate::model::{domain, request};
use crate::service::AppState;

#[tracing::instrument]
pub async fn find_all(conn: &DatabaseConnection) {
    let logs: Vec<Model> = request::Entity::find().all(conn).await.unwrap();
    info!("All logs found: {:?}", logs);
    // TODO(costi): function returns nothing
}

#[tracing::instrument]
pub async fn verify_domain(url: &String, conn: &DatabaseConnection) -> bool {
    info!("Domain being verified: {}", url);
    let return_value = domain::Entity::find_by_id(url)
        .one(conn)
        .await
        .unwrap()
        .is_some();

    info!(
        "Database query complete, domain verification output: {}",
        return_value
    );

    return_value
}

#[tracing::instrument]
pub async fn find_by_user(user_ip: &String, conn: &DatabaseConnection) -> Vec<Model> {
    info!("Finding user from database: {}", user_ip);
    match request::Entity::find()
        .filter(request::Column::Ip.eq(user_ip))
        .order_by_desc(request::Column::Timestamp)
        .all(conn)
        .await
    {
        Ok(v) => {
            info!("Successfully found request by ip");
            v
        }
        Err(e) => {
            error!("Error finding request from database: {}", e);
            vec![] // TODO(costi): empty output when database fails, unwrap was used, so this atleast is better
        }
    }
}

#[tracing::instrument]
pub async fn save_request(
    report: &web::Json<Report>,
    blocked: bool,
    db: &web::Data<AppState>,
) -> Result<Model, DbErr> {
    let incoming_request = crate::model::request::ActiveModel {
        id: ActiveValue::NotSet,
        ip: ActiveValue::Set(report.ip.to_string()),
        client_host: report
            .client_host
            .clone()
            .map_or(ActiveValue::Set("".to_string()), ActiveValue::Set),
        client_port: report
            .client_port
            .clone()
            .map_or(ActiveValue::Set("".to_string()), ActiveValue::Set),
        client_user: report
            .client_user
            .clone()
            .map_or(ActiveValue::Set("".to_string()), ActiveValue::Set),
        client_locale: report
            .client_locale
            .clone()
            .map_or(ActiveValue::Set("".to_string()), ActiveValue::Set),
        session: report
            .session
            .clone()
            .map_or(ActiveValue::Set("".to_string()), ActiveValue::Set),
        cookies: report
            .cookies
            .clone()
            .map_or(ActiveValue::Set("".to_string()), ActiveValue::Set),
        request_uri: report
            .request_uri
            .clone()
            .map_or(ActiveValue::Set("".to_string()), ActiveValue::Set),
        request_url: ActiveValue::Set(report.request_url.to_string()),
        request_method: ActiveValue::Set(report.request_method.to_string()),
        request_header: ActiveValue::Set(report.request_header.to_string()),
        request_protocol: ActiveValue::Set(report.request_protocol.to_string()),
        request_scheme: ActiveValue::Set(report.request_scheme.to_string()),
        blocked: ActiveValue::Set(blocked),
        timestamp: ActiveValue::Set(Local::now()),
        domain_id: ActiveValue::Set(report.domain_id.clone()),
    };

    info!("Saving request to database: {:?}", incoming_request);
    incoming_request.insert(&db.conn).await
}
