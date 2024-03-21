use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr};
use sqlx::types::chrono::Local;
use uuid::Uuid;
use crate::data_transfer_object::new_domain::NewDomain;
use crate::model::request::Model;
use crate::service::AppState;

pub async fn save_domain(new_domain: &web::Json<NewDomain>, db: web::Data<AppState>) -> Result<crate::model::domain::Model, DbErr> {
    let incoming_domain = crate::model::domain::ActiveModel{
        id: ActiveValue::Set(Uuid::new_v4()),
        url: ActiveValue::Set(new_domain.url.clone()),
        name: ActiveValue::Set(new_domain.name.clone()),
        timestamp: ActiveValue::Set(Local::now()),
    };
    incoming_domain.insert(&db.conn).await
}