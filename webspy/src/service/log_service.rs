use sea_orm::{DatabaseConnection, EntityTrait};
use crate::model::request;
use crate::model::request::{Model as Log, Model};

pub async fn find_all(conn: &DatabaseConnection){
    let logs: Vec<Model> = request::Entity::find().all(conn).await.unwrap();
    println!("{:?}", logs);
}