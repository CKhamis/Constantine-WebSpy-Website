use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// helpful resource: https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "logs")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id:i32,
    #[sea_orm(column_type = "Text")]
    pub text: String
}

impl ActiveModelBehavior for ActiveModel {}

// helpful info: https://docs.rs/sea-orm/0.12.14/sea_orm/derive.DeriveRelation.html
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}