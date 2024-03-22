use std::sync::RwLock;
use handlebars::Handlebars;
use lazy_static::lazy_static;

pub mod controller;
pub mod model;
pub mod util;
pub mod service;
pub mod data_transfer_object;

lazy_static!{
    pub static ref HANDLEBARS_TEMPLATE:RwLock<Handlebars<'static>> = RwLock::new(Handlebars::new());
}
