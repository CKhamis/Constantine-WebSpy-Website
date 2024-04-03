use std::net::{IpAddr, Ipv6Addr};
use std::str::FromStr;
use actix_web::web;
use chrono::NaiveDate;
use sea_orm::prelude::DateTimeLocal;
use sea_orm::{ConnectionTrait, DatabaseBackend, QueryResult, Statement};
use crate::service::AppState;

pub async fn daily_activity(db: &web::Data<AppState>) -> Vec<(DateTimeLocal, i32)> {
    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        "SELECT CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            GROUP BY CAST(DATE(timestamp) AS datetime)
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;"
    )).await.unwrap();


    // NOTE: there is an issue with this function involving the timezone of MySQL and WebSpy.
    // This can lead to differences in direct queries and WebSpy queries
    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn daily_activity_by_user(ip_address: &str, db: &web::Data<AppState>) -> Vec<(DateTimeLocal, i32)> {
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        return vec![];
    };

    println!("{:?}", format!("
            SELECT CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            WHERE ip = {}
            GROUP BY CAST(DATE(timestamp) AS datetime) ORDER BY CAST(DATE(timestamp) AS datetime) DESC;", ip_address));

    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        format!("SELECT CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            WHERE ip = '{}'
            GROUP BY CAST(DATE(timestamp) AS datetime)
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;", ip_address).replace('\n', "")
    )).await.unwrap();


    // NOTE: there is an issue with this function involving the timezone of MySQL and WebSpy.
    // This can lead to differences in direct queries and WebSpy queries
    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}