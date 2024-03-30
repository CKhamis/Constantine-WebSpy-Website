use actix_web::web;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::data_transfer_object::new_user::NewUser;
use crate::data_transfer_object::new_domain::NewDomain;
use crate::model::{user};
use crate::model::user::Model;
use crate::service::AppState;

pub async fn user_check(ip:&String, db: &web::Data<AppState>) -> Result<Option<Model>, DbErr> {
    user::Entity::find_by_id(ip).one(&db.conn).await
}

pub async fn all_users(db: &web::Data<AppState>) -> Vec<user::Model> {
    user::Entity::find().all(&db.conn).await.unwrap()
}

pub async fn new_user(new_user: NewUser, db: &web::Data<AppState>) -> Result<user::Model, DbErr> {
    let constructed_model = crate::model::user::ActiveModel{
        ip: ActiveValue::Set(new_user.ip.clone()),
        nickname: ActiveValue::NotSet,
        reason: new_user.message.map_or(ActiveValue::NotSet, |a| {ActiveValue::Set(Option::from(a))}),
        first_seen: ActiveValue::Set(Local::now()),
        expire: new_user.expire.map_or(ActiveValue::NotSet, |a| {ActiveValue::Set(Option::from(a))}),
        tags: ActiveValue::NotSet,
    };
    constructed_model.insert(&db.conn).await
}