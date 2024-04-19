use crate::data_transfer_object::new_domain::NewDomain;
use crate::model::domain;
use crate::service::AppState;
use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use sqlx::types::chrono::Local;
use uuid::Uuid;

pub async fn save_domain(
    new_domain: &web::Json<NewDomain>,
    db: web::Data<AppState>,
) -> Result<domain::Model, DbErr> {
    let incoming_domain = crate::model::domain::ActiveModel {
        domain: ActiveValue::Set(new_domain.url.clone()),
        name: ActiveValue::Set(new_domain.name.clone()),
        timestamp: ActiveValue::Set(Local::now()),
    };
    incoming_domain.insert(&db.conn).await
}

pub async fn get_domains(db: web::Data<AppState>) -> Vec<domain::Model> {
    domain::Entity::find().all(&db.conn).await.unwrap()
}
