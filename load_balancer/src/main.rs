mod route_factory;

use crate::route_factory::ui::home_page::home;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::println;

#[actix_web::get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// todo: handle errors from handlers
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(move || {
        App::new()
            .service(web::scope("/").service(index))
            .service(web::scope("/app").service(home))
    })
    .bind("0.0.0.0:8080")
    .expect("Failed to bind to port 8080")
    .run()
    .await
}
