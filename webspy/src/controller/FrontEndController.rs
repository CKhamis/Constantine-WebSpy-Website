use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use crate::util::template_config::template_validity;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    let mut reg = Handlebars::new();
    template_validity(reg.render_template(include_str!("../main.rs"), &json!({"name": "foo"})))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}