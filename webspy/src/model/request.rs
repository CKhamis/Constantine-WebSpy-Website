use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// helpful resource: https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "request")]
pub struct Model{
    #[sea_orm(primary_key, auto_increment = true)]
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
    #[sea_orm(column_type = "Timestamp")]
    pub timestamp:DateTimeLocal,
    pub domain_id: u64,
}

impl ActiveModelBehavior for ActiveModel {}

// helpful info: https://docs.rs/sea-orm/0.12.14/sea_orm/derive.DeriveRelation.html
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Domain,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Domain => Entity::belongs_to(super::domain::Entity)
                .from(Column::DomainId)
                .to(super::domain::Column::Id)
                .into(),
        }
    }
}

impl Related<super::domain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Domain.def()
    }
}