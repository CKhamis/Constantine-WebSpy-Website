use actix_web::{HttpResponse, post, Responder, web};
use handlebars::Handlebars;
use crate::data_transfer_object::report::Report;

//#[post("/report")]
pub async fn report_request(report: web::Json<Report>) -> impl Responder {
    let mut reg = Handlebars::new();
    println!("{:?}", report.request_url);
    HttpResponse::Ok().body("Hey there!")
}