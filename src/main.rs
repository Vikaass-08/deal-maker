#![allow(unused)]
use std::io;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod routes;
use routes::agreement::{get_agreement, save_agreement};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/agreement")
                    .service(get_agreement)
                    .service(save_agreement)
            )
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Home Page");
}