use crate::service::AppState;
use actix_web::web;
use sea_orm::prelude::DateTimeLocal;
use sea_orm::{ConnectionTrait, DatabaseBackend, QueryResult, Statement};
use std::net::{IpAddr, Ipv6Addr};
use std::str::FromStr;
use tracing::{info, warn};

#[tracing::instrument]
pub async fn daily_activity(db: &web::Data<AppState>) -> Vec<(DateTimeLocal, i32)> {
    info!("Getting daily activity from database");
    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            "SELECT CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            GROUP BY CAST(DATE(timestamp) AS datetime)
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;",
        ))
        .await
        .unwrap(); // TODO(costi):  handle this unwrap

    info!("Successfully got response from database query");

    // NOTE: there is an issue with this function involving the timezone of MySQL and WebSpy.
    // This can lead to differences in direct queries and WebSpy queries
    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}

#[tracing::instrument]
pub async fn domain_activity(db: &web::Data<AppState>) -> Vec<(String, i64)> {
    info!("Getting domain activity from database");
    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            "SELECT domain_id, count(*)
            FROM request
            GROUP BY domain_id
            ORDER BY domain_id;",
        ))
        .await
        .unwrap(); // TODO(costi):  handle this unwrap

    info!("Successfully got response from database query");

    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}

#[tracing::instrument]
pub async fn endpoint_frequency(db: &web::Data<AppState>) -> Vec<(String, String, String, i32)> {
    info!("Getting endpoint frequency from database");
    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            "
            SELECT domain_id, request_uri, request_method, COUNT(request_uri) AS frequency
            FROM request
            GROUP BY domain_id, request_method, request_uri
            ORDER BY frequency DESC;",
        ))
        .await
        .unwrap(); // TODO(costi):  handle this unwrap

    info!("Successfully got response from database query");

    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}

#[tracing::instrument]
pub async fn daily_activity_by_user(
    ip_address: &str,
    db: &web::Data<AppState>,
) -> Vec<(DateTimeLocal, i32)> {
    info!("Getting daily activity by user from database");
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        warn!(
            "Ip address for user: {} was unable to be cast into an ipv4 or ipv6",
            ip_address
        );
        return vec![];
    };

    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            format!(
                "SELECT CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            WHERE ip = '{}'
            GROUP BY CAST(DATE(timestamp) AS datetime)
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;",
                ip_address
            ),
        ))
        .await
        .unwrap();

    info!("Successfully got response from database");

    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}

#[tracing::instrument]
pub async fn daily_activity_by_user_by_domain(
    ip_address: &str,
    db: &web::Data<AppState>,
) -> Vec<(String, DateTimeLocal, i32)> {
    info!("Getting daily activity by user by domain from database");
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        warn!(
            "Ip address for user: {} was unable to be cast into an ipv4 or ipv6",
            ip_address
        );
        return vec![];
    };

    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            format!(
                "
            SELECT domain_id, CAST(DATE(timestamp) AS datetime) AS date, COUNT(*) AS total_requests
            FROM web_spy.request
            WHERE ip = '{}'
            GROUP BY domain_id, CAST(DATE(timestamp) AS datetime)
            ORDER BY CAST(DATE(timestamp) AS datetime) DESC;",
                ip_address
            ),
        ))
        .await
        .unwrap();

    info!("Successfully got response from database");

    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}

#[tracing::instrument]
pub async fn domain_activity_by_user(
    ip_address: &str,
    db: &web::Data<AppState>,
) -> Vec<(String, i64)> {
    info!("Getting domain activity by user from database");
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        warn!(
            "Ip address for user: {} was unable to be cast into an ipv4 or ipv6",
            ip_address
        );
        return vec![];
    };

    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            format!(
                "SELECT domain_id, count(*)
            FROM request
            WHERE ip = '{}'
            GROUP BY domain_id
            ORDER BY domain_id;",
                ip_address
            ),
        ))
        .await
        .unwrap();

    info!("Successfully got response from database");

    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}

#[tracing::instrument]
pub async fn endpoint_frequency_by_user(
    ip_address: &str,
    db: &web::Data<AppState>,
) -> Vec<(String, String, String, i32)> {
    info!("Getting endpoint frequency by user from database");
    // Check if ip address is valid
    if IpAddr::from_str(ip_address).is_err() && Ipv6Addr::from_str(ip_address).is_err() {
        warn!(
            "Ip address for user: {} was unable to be cast into an ipv4 or ipv6",
            ip_address
        );
        return vec![];
    };

    let query_result_list: Vec<QueryResult> = db
        .conn
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            format!(
                "
            SELECT domain_id, request_uri, request_method, COUNT(request_uri) AS frequency
            FROM request
            WHERE ip = '{}'
            GROUP BY domain_id, request_method, request_uri
            ORDER BY frequency DESC;",
                ip_address
            ),
        ))
        .await
        .unwrap();

    info!("Successfully got response from database");

    query_result_list
        .iter()
        .filter_map(|query_result| query_result.try_get_many_by_index().ok())
        .collect()
}
