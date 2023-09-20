use std::string;
use actix_web::error::ErrorNotFound;
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{Lender, NewLender};
use crate::database::queries::lender_queries::{create_lender_query, login_lender_query};
use crate::types::{CreateLenderReq, LoginLenderReq, LoginLenderResp, CreateLenderResp};


#[post("/create")]
pub async fn create_lender(req: web::Json<CreateLenderReq>) -> Result<impl Responder> {

  let create_user:Result<Lender, String> = create_lender_query(&req);
  match create_user {
      Ok(value) => return Ok(web::Json(
        CreateLenderResp {
            org_name: value.org_name,
            email: value.email,
            created_at: value.created_at
        }
      )),
      Err(err) => return Err(ErrorNotFound(err))
  }
}


#[post("/login")]
pub async fn login_lender(req: web::Json<LoginLenderReq>) -> Result<impl Responder> {

  let login_lender:Result<LoginLenderResp, String> = login_lender_query(&req);
  match login_lender {
      Ok(value) => return Ok(web::Json(value)),
      Err(err) => return Err(ErrorNotFound(err))
  }
}