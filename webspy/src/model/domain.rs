use sea_orm::PrimaryKeyTrait;
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EntityTrait, EnumIter, Related, RelationDef, RelationTrait};
use sea_orm::prelude::DateTimeLocal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "domain")]
pub struct Model{
    #[sea_orm(primary_key, auto_increment = false)]
    pub id:u64,
    #[sea_orm(column_type = "Text")]
    pub name:String,
    #[sea_orm(column_type = "Text")]
    pub url:String,
    #[sea_orm(column_type = "Timestamp")]
    pub timestamp:DateTimeLocal,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, Deserialize, Serialize)]
pub enum Relation {
    Request,
}

//todo: use macro?
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Request => crate::model::domain::Entity::has_many(super::request::Entity).into(),
        }
    }
}

impl Related<super::request::Entity> for crate::model::domain::Entity {
    fn to() -> RelationDef {
        Relation::Request.def()
    }
}