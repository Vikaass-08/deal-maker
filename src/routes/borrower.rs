use std::string;
use actix_web::error::{ErrorNotAcceptable, ErrorNotFound};
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{Users, NewUsers};
use crate::database::queries::users_queries::{create_user_query, login_user_query};
use crate::types::{CreateUserReq, LoginUserReq, LoginUserResp, CreateUserResp};


#[post("/create")]
pub async fn create_user(req: web::Json<CreateUserReq>) -> Result<impl Responder> {

  let create_user:Result<Users, String> = create_user_query(&req);
  match create_user {
      Ok(value) => return Ok(web::Json(
        CreateUserResp {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            created_at: value.created_at
        }
      )),
      Err(err) => return Err(ErrorNotFound(err))
  }
}

#[post("/login")]
pub async fn login_user(req: web::Json<LoginUserReq>) -> Result<impl Responder> {

  let login_user:Result<LoginUserResp, String> = login_user_query(&req);
  match login_user {
      Ok(value) => return Ok(web::Json(value)),
      Err(err) => return Err(ErrorNotFound(err))
  }
}