use std::net::{IpAddr, Ipv6Addr};
use std::str::FromStr;
use actix_web::web;
use chrono::NaiveDate;
use sea_orm::prelude::DateTimeLocal;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityOrSelect, EntityTrait, QueryResult, QuerySelect, Statement};
use crate::model::request;
use crate::model::request::Model;
use crate::service::AppState;

pub async fn unique_users_per_domain(db: &web::Data<AppState>) -> Vec<(String, i64)> {
    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        "
            SELECT domain_id, COUNT(DISTINCT ip) AS unique_visitors
            FROM web_spy.request
            GROUP BY domain_id
            ORDER BY unique_visitors;"
    )).await.unwrap();

    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn daily_activity(db: &web::Data<AppState>) -> Vec<(DateTimeLocal, String, i32)> {
    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        "SELECT CAST(DATE(timestamp) AS datetime) AS date, domain_id, COUNT(*) AS total_requests
            FROM web_spy.request
            GROUP BY CAST(DATE(timestamp) AS datetime), domain_id
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;"
    )).await.unwrap();


    // NOTE: there is an issue with this function involving the timezone of MySQL and WebSpy.
    // This can lead to differences in direct queries and WebSpy queries
    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn domain_activity(db: &web::Data<AppState>) -> Vec<(String, i64)> {
    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        "SELECT domain_id, count(*)
            FROM request
            GROUP BY domain_id
            ORDER BY domain_id;"
    )).await.unwrap();

    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn endpoint_frequency(db: &web::Data<AppState>) -> Vec<(String, String, String, i32)> {
    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        "
            SELECT domain_id, request_uri, request_method, COUNT(request_uri) AS frequency
            FROM request
            GROUP BY domain_id, request_method, request_uri
            ORDER BY frequency DESC;"
    )).await.unwrap();

    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn daily_activity_by_user(ip_address: &str, db: &web::Data<AppState>) -> Vec<(DateTimeLocal, i32)> {
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        return vec![];
    };

    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        format!("SELECT CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            WHERE ip = '{}'
            GROUP BY CAST(DATE(timestamp) AS datetime)
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;", ip_address)
    )).await.unwrap();

    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn daily_activity_by_user_by_domain(ip_address: &str, db: &web::Data<AppState>) -> Vec<(String, DateTimeLocal, i32)> {
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        return vec![];
    };

    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        format!("
            SELECT domain_id, CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            WHERE ip = '{}'
            GROUP BY domain_id, CAST(DATE(timestamp) AS datetime)
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;", ip_address)
    )).await.unwrap();

    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn domain_activity_by_user(ip_address: &str, db: &web::Data<AppState>) -> Vec<(String, i64)> {
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        return vec![];
    };

    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        format!("SELECT domain_id, count(*)
            FROM request
            WHERE ip = '{}'
            GROUP BY domain_id
            ORDER BY domain_id;", ip_address)
    )).await.unwrap();

    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}

pub async fn endpoint_frequency_by_user(ip_address: &str, db: &web::Data<AppState>) -> Vec<(String, String, String, i32)> {
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        return vec![];
    };

    let query_result_list: Vec<(QueryResult)> = db.conn.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        format!("
            SELECT domain_id, request_uri, request_method, COUNT(request_uri) AS frequency
            FROM request
            WHERE ip = '{}'
            GROUP BY domain_id, request_method, request_uri
            ORDER BY frequency DESC;", ip_address)
    )).await.unwrap();

    query_result_list.iter().filter_map(|query_result| {
        query_result.try_get_many_by_index().ok()
    }).collect()
}