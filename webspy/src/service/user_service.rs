use actix_web::web;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationDef, RelationTrait};
use uuid::Uuid;
use crate::data_transfer_object::new_user::NewUser;
use crate::data_transfer_object::new_domain::NewDomain;
use crate::model::{request, user};
use crate::model::user::Model;
use crate::service::AppState;

pub async fn user_check(ip:&String, db: &web::Data<AppState>) -> Result<Option<Model>, DbErr> {
    user::Entity::find_by_id(ip).one(&db.conn).await
}

pub async fn all_users(db: &web::Data<AppState>) -> Vec<user::Model> {
    user::Entity::find().order_by_desc(user::Column::FirstSeen).all(&db.conn).await.unwrap()
}

pub async fn active_users(db: &web::Data<AppState>) -> Vec<(String, i64)> {
    println!("{:?}", request::Entity::find()
        .select_only()
        .column(request::Column::Ip)
        .column(request::Column::Timestamp.max())
        .group_by(request::Column::Ip)
        .order_by_desc(request::Column::Timestamp)
        .all(&db.conn).await);
    todo!() //todo: this does not work. it gives an error saying the ip column does not exist in the request table, but it is
}

pub async fn banned_users(db: &web::Data<AppState>) -> Vec<user::Model> {
    let now = Local::now();
    user::Entity::find()
        .filter(user::Column::Expire.is_not_null())
        .filter(user::Column::Expire.gt(now))
        .all(&db.conn).await.unwrap()
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