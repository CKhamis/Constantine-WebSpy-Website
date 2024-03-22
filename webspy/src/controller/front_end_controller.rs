use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use crate::HANDLEBARS_TEMPLATE;
use crate::service::AppState;
use crate::service::report_service::find_all;
use crate::util::template_config::template_validity;

#[get("/")]
pub async fn hello(db: web::Data<AppState>) -> impl Responder {
    let reg = HANDLEBARS_TEMPLATE.read().unwrap();
    let model = json!({
        "title": "Home"
    });

    let rendered_content = reg.render("home", &model)
        .expect("Failed to render template");

    // Assuming `template_validity` is a function that checks the validity of the rendered template
    HttpResponse::Ok().body(rendered_content)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    let mut reg = Handlebars::new();
    template_validity(reg.render_template(include_str!("../main.rs"), &json!({"name": "foo"})))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}