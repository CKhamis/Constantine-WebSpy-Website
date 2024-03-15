use actix_web::{HttpResponse, Responder};
use handlebars::RenderError;
use serde_json::json;

pub fn template_validity(input:Result<String, RenderError>) -> impl Responder {
    match input{
        Ok(body) => {
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            println!("{}", e);
            HttpResponse::ImATeapot().body(format!("OOOOOOOPS! There was an error :( {}", e))
        }
    }
}