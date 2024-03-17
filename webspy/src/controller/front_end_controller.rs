use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use crate::service::AppState;
use crate::service::request_service::find_all;
use crate::util::template_config::template_validity;

#[get("/")]
pub async fn hello(db: web::Data<AppState>) -> impl Responder {
    find_all(&db.conn).await;
    let mut reg = Handlebars::new();
    //HttpResponse::Ok().body("Hello world!")
    template_validity(reg.render_template(
        include_str!("../../resources/template/main/Index.html"),
        &json!({"header": include_str!("../../resources/template/components/Header.html"), "logs": "rat>>>>>"})))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    let mut reg = Handlebars::new();
    template_validity(reg.render_template(include_str!("../main.rs"), &json!({"name": "foo"})))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}