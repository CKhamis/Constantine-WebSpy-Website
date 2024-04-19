use crate::HANDLEBARS_TEMPLATE;
use actix_web::{HttpResponse, Responder};
use handlebars::RenderError;
use serde_json::json;

pub fn template_resources() {
    let mut reg = HANDLEBARS_TEMPLATE.write().unwrap();
    reg.register_partial(
        "header",
        include_str!("../../resources/template/components/Header.hbs"),
    )
    .expect("heading file not found");
    reg.register_partial(
        "navigation",
        include_str!("../../resources/template/components/Navigation.hbs"),
    )
    .expect("heading file not found");
    reg.register_partial(
        "scripts",
        include_str!("../../resources/template/components/Scripts.hbs"),
    )
    .expect("heading file not found");
    reg.register_partial(
        "footer",
        include_str!("../../resources/template/components/Footer.hbs"),
    )
    .expect("heading file not found");
    reg.register_partial(
        "dashboard_navigation",
        include_str!("../../resources/template/components/Dashboard_Navigation.hbs"),
    )
    .expect("heading file not found");

    reg.register_template_string(
        "home",
        include_str!("../../resources/template/main/Index.hbs"),
    )
    .expect("home file not found");
    reg.register_template_string(
        "dashboard",
        include_str!("../../resources/template/main/Dashboard.hbs"),
    )
    .expect("home file not found");
}

pub fn template_validity(input: Result<String, RenderError>) -> impl Responder {
    match input {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            println!("{}", e);
            HttpResponse::ImATeapot().body(format!("OOOOOOOPS! There was an error :( {}", e))
        }
    }
}
