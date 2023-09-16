#![allow(unused)]
use std::io;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/routes")
                        .service(routes::add)
            )
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world Vikas!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}