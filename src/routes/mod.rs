
use actix_web::{get, post, HttpResponse, Responder, web, Result};
use serde::Serialize;
pub mod document;
pub mod borrower;
pub mod lender;
pub mod document_request;

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Document Server is up");
}