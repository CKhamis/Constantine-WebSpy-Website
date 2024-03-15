use actix_web::{App, HttpServer, web};
use webspy::controller::controller_prelude::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("terence");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}