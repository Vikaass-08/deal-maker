
use std::string;
use actix_web::error::ErrorNotFound;
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{Agreement, NewAgreement, AgreementList};
use crate::database::agreement_queries::{get_agreement_query, create_agreement_query};

#[get("/get")]
pub async fn get_agreement() -> Result<impl Responder> {
    let get_query: Result<AgreementList, String> = get_agreement_query();
    match get_query {
        Ok(value) => return Ok(web::Json(value)),
        Err(err) => return Err(ErrorNotFound(err)),
    };
}

#[post("/save")]
pub async fn save_agreement(req: web::Json<Agreement>) -> Result<impl Responder> {
  let save_agreement:Result<Agreement, String> = create_agreement_query(&req.agreement_data.to_string(), &req.data_type.to_string());

  match save_agreement {
      Ok(value) => return Ok(web::Json(value)),
      Err(err) => return Err(ErrorNotFound(err))
  }
}