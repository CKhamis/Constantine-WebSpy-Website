use sea_orm::PrimaryKeyTrait;
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EntityTrait, EnumIter, Related, RelationDef, RelationTrait};
use sea_orm::prelude::DateTimeLocal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "ban")]
pub struct Model{
    #[sea_orm(primary_key, auto_increment = false)]
    pub id:u64,
    #[sea_orm(column_type = "Text")]
    pub ip:String, //make this the primary key eventually?
    #[sea_orm(column_type = "Text")]
    pub reason:String,
    #[sea_orm(column_type = "Timestamp")]
    pub expire:DateTimeLocal,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}