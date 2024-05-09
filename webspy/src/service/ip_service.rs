use actix_web::web;
use chrono::Local;
use sea_orm::prelude::DateTimeLocal;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QueryResult, Statement,
};

use crate::data_transfer_object::new_ip::newIp;
use crate::model::ip;
use crate::model::ip::Model;
use crate::service::AppState;
use crate::util::threat_level::DangerLevel;

pub async fn ip_check(ip: &String, db: &web::Data<AppState>) -> Result<Option<Model>, DbErr> {
    ip::Entity::find_by_id(ip).one(&db.conn).await
}

pub async fn all_users(db: &web::Data<AppState>) -> Vec<ip::Model> {
    ip::Entity::find()
        .order_by_desc(ip::Column::FirstSeen)
        .all(&db.conn)
        .await
        .unwrap()
}

// type CoolRow = Vec<(String, Option<DateTimeLocal>)>;
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

pub async fn banned_users(db: &web::Data<AppState>) -> Vec<ip::Model> {
    let now = Local::now();
    ip::Entity::find()
        .filter(ip::Column::Expire.is_not_null())
        .filter(ip::Column::Expire.gt(now))
        .all(&db.conn)
        .await
        .unwrap()
}

pub async fn new_ip(new_user: newIp, db: &web::Data<AppState>) -> Result<ip::Model, DbErr> {
    let constructed_model = crate::model::ip::ActiveModel {
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
