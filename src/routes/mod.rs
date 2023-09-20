
use actix_web::{get, post, HttpResponse, Responder, web, Result};
use serde::Serialize;
pub mod document;
pub mod borrower;
pub mod lender;

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Agreement Server is up");
}