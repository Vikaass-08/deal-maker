use std::string;
use actix_web::error::{ErrorUnauthorized, ErrorNotFound};
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};
use crate::types::{DocumentRequestUsersAPIReq, DocumentRequestAPIResp, TokenClaims, DocumentRequestLenderAPIReq, DocStatusCode::{REJECTED, ACCEPTED}};
use crate::database::queries::document_request_queries::{document_request_status_query_users, document_request_status_query_lender};




#[post("/get-or-create-req")]
pub async fn user_doc_request_status(req: web::Json<DocumentRequestUsersAPIReq>, token_verify: Option<web::ReqData<TokenClaims>>) -> Result<impl Responder> {
  match token_verify {
    Some(user) => {
      let save_document:Result<DocumentRequestAPIResp, String> = document_request_status_query_users(&req);
      match save_document {
        Ok(value) => return Ok(web::Json(value)),
        Err(err) => return Err(ErrorNotFound(err))
      }
    }
    None => return Err(ErrorUnauthorized("Access Denied"))
  }
}




#[get("/get-all-request")]
pub async fn lender_doc_request_status(req: web::Json<DocumentRequestLenderAPIReq>, token_verify: Option<web::ReqData<TokenClaims>>) -> Result<impl Responder> {
  match token_verify {
    Some(user) => {
      let save_document:Result<DocumentRequestAPIResp, String> = document_request_status_query_lender(false, &req);
      match save_document {
        Ok(value) => return Ok(web::Json(value)),
        Err(err) => return Err(ErrorNotFound(err))
      }
    }
    None => return Err(ErrorUnauthorized("Access Denied"))
  }
}





#[post("/update-request")]
pub async fn lender_update_doc_status(req: web::Json<DocumentRequestLenderAPIReq>, token_verify: Option<web::ReqData<TokenClaims>>) -> Result<impl Responder> {
  match token_verify {
    Some(user) => {
      let doc_status = req.status.to_uppercase();
      if !(doc_status == ACCEPTED.to_string() || doc_status == REJECTED.to_string()){
        return return Err(ErrorNotFound("Please Enter a valid Status"))
      }

      let save_document:Result<DocumentRequestAPIResp, String> = document_request_status_query_lender(true, &req);
      match save_document {
        Ok(value) => return Ok(web::Json(value)),
        Err(err) => return Err(ErrorNotFound(err))
      }
    }
    None => return Err(ErrorNotFound("Access Denied!!"))
  }
}