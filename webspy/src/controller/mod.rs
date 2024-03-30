pub mod front_end_controller;
pub mod report_controller;
pub mod domain_controller;
pub mod user_controller;
pub mod controller_prelude{
    pub use crate::controller::front_end_controller::*;
}