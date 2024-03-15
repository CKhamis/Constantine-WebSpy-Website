use sea_orm::{DatabaseConnection, EntityTrait};
use crate::model::log;
use crate::model::log::{Model as Log, Model};

pub async fn find_all(conn: &DatabaseConnection){
    let logs: Vec<Model> = log::Entity::find().all(conn).await.unwrap();
    println!("{:?}", logs);
}