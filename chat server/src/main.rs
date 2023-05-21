use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    println!("Hello, world!");
    HttpResponse::Ok().body("Hello, world!")
}

async fn greeting(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/greeting/{name}", web::get().to(greeting))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
