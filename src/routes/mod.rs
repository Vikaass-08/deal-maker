
pub mod agreement;
use actix_web::{get, post, HttpResponse, Responder, web, Result};
use serde::Serialize;

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Agreement Server is up");
}