use std::string;
use actix_web::error::{ErrorNotAcceptable, ErrorNotFound};
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{Users, NewUsers};
use crate::database::users_queries::create_user_query;

pub enum UserType {
    LENDER,
    BORROWER
}

impl UserType {
    fn to_string(&self) -> &str {
        match &self {
            Self::LENDER => "LENDER",
            Self::BORROWER => "BORROWER"
        }
    }
}

#[post("/create")]
pub async fn create_user(req: web::Json<Users>) -> Result<impl Responder> {

  let create_user:Result<Users, String> = create_user_query(&req);
  match create_user {
      Ok(value) => return Ok(web::Json(value)),
      Err(err) => return Err(ErrorNotFound(err))
  }
}