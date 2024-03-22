pub mod front_end_controller;
pub mod report_controller;
pub mod domain_controller;
pub mod ban_controller;
pub mod file_controller;
pub mod controller_prelude{
    pub use crate::controller::front_end_controller::*;
}