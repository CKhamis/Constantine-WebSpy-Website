use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use sea_orm::prelude::DateTimeLocal;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "request")]
pub struct Model{
    #[sea_orm(primary_key, auto_increment = true)]
    pub id:i32,
    #[sea_orm(column_type = "Text")]
    pub name:String,
    #[sea_orm(column_type = "Text")]
    pub url:String,
    #[sea_orm(column_type = "Text", unique)]
    pub api_key:String,
    #[sea_orm(column_type = "Timestamp")]
    pub timestamp:DateTimeLocal,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}