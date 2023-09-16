
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn add() -> impl Responder {
    HttpResponse::Ok().body("SERVER IS UP")
}