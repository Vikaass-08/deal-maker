use std::string;
use actix_web::error::{ErrorNotAcceptable, ErrorNotFound};
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{User, NewUser};
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

#[post("/createuser")]
pub async fn create_user(req: web::Json<User>) -> Result<impl Responder> {

	if !!!(req.user_type.to_uppercase() == UserType::LENDER.to_string() || req.user_type == UserType::BORROWER.to_string()) {
		return Err(ErrorNotAcceptable("User Type Mismatch"));
	}
  let create_user:Result<User, String> = create_user_query(&req);

  match create_user {
      Ok(value) => return Ok(web::Json(value)),
      Err(err) => return Err(ErrorNotFound(err))
  }
}