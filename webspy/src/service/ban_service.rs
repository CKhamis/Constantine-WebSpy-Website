use actix_web::web;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::data_transfer_object::new_ban::NewBan;
use crate::data_transfer_object::new_domain::NewDomain;
use crate::model::{ban};
use crate::model::ban::Model;
use crate::service::AppState;

pub async fn ban_check(ip:&String, db: &web::Data<AppState>) -> Option<Model> {
    ban::Entity::find().filter(ban::Column::Ip.contains(ip)).one(&db.conn).await.unwrap_or(None)
}

pub async fn save_ban(new_ban: &web::Json<NewBan>, db: &web::Data<AppState>) -> Result<ban::Model, DbErr> {
    let constructed_model = crate::model::ban::ActiveModel{
        id: ActiveValue::Set(Local::now().timestamp().unsigned_abs()),
        ip: ActiveValue::Set(new_ban.ip.clone()),
        reason: ActiveValue::Set(new_ban.message.clone()),
        expire: ActiveValue::Set(new_ban.expire),
    };
    constructed_model.insert(&db.conn).await
}