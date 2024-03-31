use actix_web::web;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend, DbBackend, DbErr, EntityTrait, JoinType, QueryFilter, QueryOrder, QueryResult, QuerySelect, QueryTrait, RelationDef, RelationTrait, Statement};
use sea_orm::prelude::DateTimeLocal;
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

pub async fn active_users(db: &web::Data<AppState>) -> Vec<(String, Option<DateTimeLocal>)> {
    let rat: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        "SELECT ip, MAX(request.timestamp) AS last_seen
            FROM web_spy.request
            GROUP BY ip
            ORDER BY last_seen DESC;"
    )).await.unwrap();

    rat.iter().filter_map(|ratr| {
        let ip:Result<String, DbErr> = ratr.try_get_by(0);// try get many by index
        let last_seen:Result<DateTimeLocal, DbErr> = ratr.try_get_by(1);
        let raet = ip.ok().map(|a| {(a, last_seen.ok())});
        raet
    }).collect()


    // println!("{:?}", request::Entity::find()
    //     .select_only()
    //     .column_as(request::Column::Ip, "ip")
    //     .expr_as_(request::Column::Timestamp.max(), "last seen")
    //     .group_by(request::Column::Ip)
    //     .order_by_desc(request::Column::Timestamp.max())
    //     .build(DbBackend::MySql)
    //     .to_string());
    // println!("{:?}", request::Entity::find()
    //     .select_only()
    //     .column_as(request::Column::Ip, "ip")
    //     .expr_as_(request::Column::Timestamp.max(), "last seen")
    //     .group_by(request::Column::Ip)
    //     .order_by_desc(request::Column::Timestamp.max())
    //     .all(&db.conn).await);
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