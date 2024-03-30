use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum DangerLevel {
    #[sea_orm(num_value = 0)]
    Threat,
    #[sea_orm(num_value = 1)]
    Suspicious,
    #[sea_orm(num_value = 2)]
    Normal,
    #[sea_orm(num_value = 3)]
    NotAssessed,
}