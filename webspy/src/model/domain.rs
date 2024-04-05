use sea_orm::prelude::DateTimeLocal;
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, EntityTrait, EnumIter, Related, RelationDef, RelationTrait};
use sea_orm::{
    ColumnDef, ColumnTrait, ColumnType, ColumnTypeTrait, DeriveActiveModel, DeriveColumn,
    DeriveEntity, DeriveModel, EntityName, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity; // add the entity struct, since we don't plan on generating this with proc macros

impl EntityName for Entity {
    // add the table name that the proc macro would have generated
    fn table_name(&self) -> &str {
        "domain"
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub domain: String,
    pub name: String,
    pub timestamp: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    // define each column en order of appearance in the model struct
    Domain,
    Name,
    Timestamp,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Domain, // define primary key manually, so we don't get any macro conflicts
            // domain can be used here now since it's a `limited string` as defined in the column trait
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;

    fn auto_increment() -> bool {
        false // we disable auto incrementing since the primary key is now a string
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, Deserialize, Serialize)]
pub enum Relation {
    Request,
}

// Implement column trait for each column defined in the model
impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            // set all the types to be used by the domain model in the database's columns
            // Column::Id => ColumnType::BigUnsigned.def(), // this column for ID gets removed since we plan on using domain as the ID instead
            Column::Domain => ColumnType::String(Some(255)).def(), // this evaluates to varchar(255)
            Column::Name => ColumnType::Text.def(),
            Column::Timestamp => ColumnType::Timestamp.def(),
        }
    }
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
