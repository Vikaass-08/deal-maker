#![allow(unused)]
use std::io;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
mod routes;
mod database;
use database::{lib, schema, models};
use diesel::expression::is_aggregate::No;
use routes::document::{get_document, save_document};
use routes::borrower::{create_user, login_user};
use routes::lender::{create_lender, login_lender};
pub mod types;
use actix_web_httpauth::middleware::HttpAuthentication;
use lib::{validator_user, validator_lender};
use types::TokenClaims;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let user_token_middleware = HttpAuthentication::bearer(validator_user);
        let lender_token_middleware = HttpAuthentication::bearer(validator_lender);
        App::new()
            .service(web::scope("/document")
                    .wrap(lender_token_middleware.clone())
                    .service(get_document)
                    .service(save_document)
            )
            .service(web::scope("/borrower")
                    .service(create_user)
                    .service(login_user)
            )
            .service(web::scope("/lender")
                    .service(create_lender)
                    .service(login_lender)
            )
            .service(web::scope("/test")
                    .wrap(user_token_middleware.clone()) 
                    .service(hello)
                )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/tokenCheck")]
async fn hello(req_user: Option<web::ReqData<TokenClaims>>) -> impl Responder {
    match req_user {
        Some(user) => {
            let id = user.id;
            return HttpResponse::Ok().json(user.id);
        }
        None => HttpResponse::Unauthorized().json("Unable to verify identity"),

    }
    // return HttpResponse::Ok().body("Home Page");
}