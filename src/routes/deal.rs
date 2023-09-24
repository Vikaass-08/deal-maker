use std::string;
use actix_web::error::{ErrorUnauthorized, ErrorNotFound};
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{Deal, NewDeal};
use crate::database::queries::deal_queries::{create_deal_query, update_deal_query};
use crate::types::{CreateDealReq, CreateDealResp, UpdateDealReq, UpdateDealResp, TokenClaims};


#[post("/create-deal")]
pub async fn create_deal(req: web::Json<CreateDealReq>, token_verify: Option<web::ReqData<TokenClaims>>) -> Result<impl Responder> {
  match token_verify {
    Some(user) => {
      let create_deal:Result<CreateDealResp, String> = create_deal_query(&req);
      match create_deal {
          Ok(value) => return Ok(return Ok(web::Json(value))),
          Err(err) => return Err(ErrorNotFound(err)),
      }
    }
    None => return Err(ErrorUnauthorized("Access Denied"))
  }
}


#[post("/update-deal")]
pub async fn update_deal(req: web::Json<UpdateDealReq>, token_verify: Option<web::ReqData<TokenClaims>>) -> Result<impl Responder> {
  match token_verify {
    Some(user) => {
      let update_deal:Result<UpdateDealResp, String> = update_deal_query(&req);
      match update_deal {
          Ok(value) => return Ok(return Ok(web::Json(value))),
          Err(err) => return Err(ErrorNotFound(err)),
      }
    }
    None => return Err(ErrorUnauthorized("Access Denied"))
  }
}