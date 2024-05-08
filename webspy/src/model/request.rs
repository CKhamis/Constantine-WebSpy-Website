use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// helpful resource: https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/
#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity; // add the entity struct, since we don't plan on generating this with proc macros

impl EntityName for crate::model::request::Entity {
    // add the table name that the proc macro would have generated
    fn table_name(&self) -> &str {
        "request"
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub ip: String,
    pub client_port: String,
    pub client_user: String,
    pub client_locale: String,
    pub user_agent: String,
    pub session: String,
    pub cookies: String,
    pub request_uri: String,
    pub request_url: String,
    pub request_method: String,
    pub request_header: String,
    pub request_protocol: String,
    pub request_scheme: String,
    pub blocked: bool,
    pub timestamp: DateTimeLocal,
    pub domain_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;

    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Ip,
    ClientPort,
    ClientUser,
    ClientLocale,
    UserAgent,
    Session,
    Cookies,
    RequestUri,
    RequestUrl,
    RequestMethod,
    RequestHeader,
    RequestProtocol,
    RequestScheme,
    Blocked,
    Timestamp,
    DomainId,
}

impl ColumnTrait for crate::model::request::Column {
    type EntityName = crate::model::request::Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Column::Id => ColumnType::Integer.def(),
            Column::Ip => ColumnType::String(Some(255)).def(),
            Column::ClientPort => ColumnType::Text.def(),
            Column::ClientUser => ColumnType::Text.def(),
            Column::ClientLocale => ColumnType::Text.def(),
            Column::UserAgent => ColumnType::Text.def(),
            Column::Session => ColumnType::Text.def(),
            Column::Cookies => ColumnType::Text.def(),
            Column::RequestUri => ColumnType::Text.def(),
            Column::RequestUrl => ColumnType::Text.def(),
            Column::RequestMethod => ColumnType::Text.def(),
            Column::RequestHeader => ColumnType::Text.def(),
            Column::RequestProtocol => ColumnType::Text.def(),
            Column::RequestScheme => ColumnType::Text.def(),
            Column::Blocked => ColumnType::Boolean.def(),
            Column::Timestamp => ColumnType::Timestamp.def(),
            Column::DomainId => ColumnType::String(Some(255)).def(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

// helpful info: https://docs.rs/sea-orm/0.12.14/sea_orm/derive.DeriveRelation.html
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Domain,
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Domain => Entity::belongs_to(super::domain::Entity)
                .from(Column::DomainId)
                .to(super::domain::Column::Domain)
                .into(),
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Ip)
                .to(super::user::Column::Ip)
                .into(),
        }
    }
}

impl Related<super::domain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Domain.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
