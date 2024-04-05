use crate::data_transfer_object::new_user::NewUser;
use crate::model::user;
use crate::model::user::Model;
use crate::service::AppState;
use crate::util::threat_level::DangerLevel;
use actix_web::web;
use chrono::Local;
use sea_orm::prelude::DateTimeLocal;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QueryResult, Statement,
};

#[tracing::instrument]
pub async fn user_check(ip: &String, db: &web::Data<AppState>) -> Result<Option<Model>, DbErr> {
    user::Entity::find_by_id(ip).one(&db.conn).await
}

#[tracing::instrument]
pub async fn all_users(db: &web::Data<AppState>) -> Vec<user::Model> {
    user::Entity::find()
        .order_by_desc(user::Column::FirstSeen)
        .all(&db.conn)
        .await
        .unwrap()
}

// type CoolRow = Vec<(String, Option<DateTimeLocal>)>;
#[tracing::instrument]
pub async fn active_users(db: &web::Data<AppState>) -> Vec<(String, Option<DateTimeLocal>)> {
    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            "SELECT ip, MAX(request.timestamp) AS last_seen
            FROM web_spy.request
            GROUP BY ip
            ORDER BY last_seen DESC;",
        ))
        .await
        .unwrap();

    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}

#[tracing::instrument]
pub async fn banned_users(db: &web::Data<AppState>) -> Vec<user::Model> {
    let now = Local::now();
    user::Entity::find()
        .filter(user::Column::Expire.is_not_null())
        .filter(user::Column::Expire.gt(now))
        .all(&db.conn)
        .await
        .unwrap()
}

#[tracing::instrument]
pub async fn new_user(new_user: NewUser, db: &web::Data<AppState>) -> Result<user::Model, DbErr> {
    let constructed_model = crate::model::user::ActiveModel {
        ip: ActiveValue::Set(new_user.ip.clone()),
        nickname: ActiveValue::NotSet,
        reason: new_user
            .message
            .map_or(ActiveValue::NotSet, |a| ActiveValue::Set(Option::from(a))),
        first_seen: ActiveValue::Set(Local::now()),
        expire: new_user
            .expire
            .map_or(ActiveValue::NotSet, |a| ActiveValue::Set(Option::from(a))),
        threat_level: ActiveValue::Set(DangerLevel::NotAssessed),
    };
    constructed_model.insert(&db.conn).await
}
