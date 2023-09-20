#![allow(unused)]
use std::io;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod routes;
mod database;
use database::{lib, schema, models};
use routes::document::{get_document, save_document};
use routes::borrower::{create_user};
pub mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/document")
                    .service(get_document)
                    .service(save_document)
            )
            .service(web::scope("/borrower")
                    .service(create_user)
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