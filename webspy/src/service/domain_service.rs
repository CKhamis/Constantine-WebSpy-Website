use actix_web::web;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use sqlx::types::chrono::Local;
use tracing::info;

use crate::data_transfer_object::new_domain::NewDomain;
use crate::model::domain;
use crate::service::AppState;

#[tracing::instrument]
pub async fn save_domain(
    new_domain: &web::Json<NewDomain>,
    db: web::Data<AppState>,
) -> Result<domain::Model, DbErr> {
    info!("Saving domain object");
    let incoming_domain = crate::model::domain::ActiveModel {
        domain: ActiveValue::Set(new_domain.url.clone()),
        name: ActiveValue::Set(new_domain.name.clone()),
        timestamp: ActiveValue::Set(Local::now()),
    };
    incoming_domain.insert(&db.conn).await
}

#[tracing::instrument]
pub async fn get_domains(db: web::Data<AppState>) -> Vec<domain::Model> {
    info!("Getting domain list");
    domain::Entity::find().all(&db.conn).await.unwrap()
}
