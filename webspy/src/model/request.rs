use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// helpful resource: https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "request")]
pub struct Model{
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "Integer")]
    pub id:i32,
    #[sea_orm(column_type = "Text")]
    pub ip:String,
    #[sea_orm(column_type = "Text")]
    pub client_host:String,
    #[sea_orm(column_type = "Text")]
    pub client_port:String,
    #[sea_orm(column_type = "Text")]
    pub client_user:String,
    #[sea_orm(column_type = "Text")]
    pub client_locale:String,
    #[sea_orm(column_type = "Text")]
    pub session:String,
    #[sea_orm(column_type = "Text")]
    pub cookies:String,
    #[sea_orm(column_type = "Text")]
    pub request_uri:String,
    #[sea_orm(column_type = "Text")]
    pub request_url:String,
    #[sea_orm(column_type = "Text")]
    pub request_method:String,
    #[sea_orm(column_type = "Text")]
    pub request_header:String,
    #[sea_orm(column_type = "Text")]
    pub request_protocol:String,
    #[sea_orm(column_type = "Text")]
    pub request_scheme:String,
}

impl ActiveModelBehavior for ActiveModel {}

// helpful info: https://docs.rs/sea-orm/0.12.14/sea_orm/derive.DeriveRelation.html
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}