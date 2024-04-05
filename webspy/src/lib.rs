// #![deny(clippy::unwrap_used)] // TODO(costi): uncomment if you dare
use handlebars::Handlebars;
use lazy_static::lazy_static;
use std::sync::RwLock;

pub mod controller;
pub mod data_transfer_object;
pub mod model;
pub mod service;
pub mod util;

lazy_static! {
    pub static ref HANDLEBARS_TEMPLATE: RwLock<Handlebars<'static>> =
        RwLock::new(Handlebars::new());
}
