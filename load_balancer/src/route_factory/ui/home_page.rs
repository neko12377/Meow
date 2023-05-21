use actix_web::{error, web, App, Either, Error, HttpResponse, HttpServer, Responder, Result};
use reqwest;
use std::println;

#[actix_web::get("/home")]
pub async fn home() -> Result<HttpResponse, Error> {
    println!("Fetching data from localhost:3000");
    let response = reqwest::get("http://chat_ui:3000")
        .await
        .map_err(error::ErrorInternalServerError)?;
    let status = response.status();
    println!("status: {:?}", status);
    println!("response: {:?}", response);
    let body = response
        .text()
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
