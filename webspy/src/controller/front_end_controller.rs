use crate::service::AppState;
use crate::util::template_config::template_validity;
use crate::HANDLEBARS_TEMPLATE;
use actix_web::{get, post, web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
#[tracing::instrument]
pub async fn index(db: web::Data<AppState>) -> impl Responder {
    let reg = HANDLEBARS_TEMPLATE.read().unwrap();
    let model = json!({
        "title": "Home",
        "version": "0.5",
        "authenticated": true,
    });

    let rendered_content = reg
        .render("home", &model)
        .expect("Failed to render template");

    // Assuming `template_validity` is a function that checks the validity of the rendered template
    HttpResponse::Ok().body(rendered_content)
}

#[get("/dashboard")]
#[tracing::instrument]
pub async fn dashboard(db: web::Data<AppState>) -> impl Responder {
    let reg = HANDLEBARS_TEMPLATE.read().unwrap();
    let model = json!({
        "title": "Dashboard",
        "authenticated": false,
    });

    let rendered_content = reg
        .render("dashboard", &model)
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
