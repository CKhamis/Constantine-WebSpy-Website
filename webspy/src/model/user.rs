use sea_orm::{ColumnDef, ColumnTrait, ColumnType, ColumnTypeTrait, DeriveActiveModel, DeriveColumn, DeriveEntity, DeriveModel, EntityName, PrimaryKeyTrait};
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EntityTrait, EnumIter, Related, RelationDef, RelationTrait};
use sea_orm::prelude::DateTimeLocal;
use serde::{Deserialize, Serialize};
use crate::util::threat_level::DangerLevel;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity; // add the entity struct, since we don't plan on generating this with proc macros

impl EntityName for Entity {
    // add the table name that the proc macro would have generated
    fn table_name(&self) -> &str {
        "user"
    }
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveModel, DeriveActiveModel)]
pub struct Model{
    pub ip:String,
    pub nickname: Option<String>,
    pub reason: Option<String>,
    pub first_seen: DateTimeLocal,
    pub expire: Option<DateTimeLocal>,
    pub threat_level: DangerLevel
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Ip,
    Nickname,
    Reason,
    FirstSeen,
    Expire,
    ThreatLevel,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Ip, // define primary key manually, so we don't get any macro conflicts
    // domain can be used here now since it's a `limited string` as defined in the column trait
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;

    fn auto_increment() -> bool {
        false // we disable auto incrementing since the primary key is now a string
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Implement column trait for each column defined in the model
impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self { // set all the types to be used by the domain model in the database's columns
            // Column::Id => ColumnType::BigUnsigned.def(), // this column for ID gets removed since we plan on using domain as the ID instead
            Column::Ip => ColumnType::String(Some(255)).def(),
            Column::Nickname => ColumnType::Text.def().nullable(),
            Column::Reason => ColumnType::Text.def().nullable(),
            Column::Expire => ColumnType::Timestamp.def().nullable(),
            Column::FirstSeen => ColumnType::Timestamp.def(),
            Column::ThreatLevel => ColumnType::Integer.def(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, Deserialize, Serialize)]
pub enum Relation {
    Request,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Request => Entity::has_many(super::request::Entity).into(),
        }
    }
}

impl Related<super::request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Request.def()
    }
}