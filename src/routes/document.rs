
use std::string;
use actix_web::error::{ErrorUnauthorized, ErrorNotFound};
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{Document, NewDocument};
use crate::database::queries::document_queries::{get_document_query, create_document_query};
use crate::types::{SaveDocumentReq, TokenClaims};




pub async fn get_document(document_id: i32, req_user: Option<web::ReqData<TokenClaims>>) -> Result<impl Responder> {
    match req_user {
        Some(user) => {
            let get_query: Result<Document, String> = get_document_query(document_id);
            match get_query {
                Ok(value) => return Ok(web::Json(value)),
                Err(err) => return Err(ErrorNotFound(err)),
            };
        }
        None => return Err(ErrorUnauthorized("Access Denied")),
    }
}




#[post("/create-document")]
pub async fn save_document(req: web::Json<SaveDocumentReq>, req_user: Option<web::ReqData<TokenClaims>>) -> Result<impl Responder> {
    match req_user {
        Some(user) => {
            let save_document:Result<Document, String> = create_document_query(
                &req.document_data.to_string(), 
                &req.document_type.to_string(), 
                &req.lender_email.to_string(),
                &req.user_email.to_string(),

            );
        
            match save_document {
                Ok(value) => return Ok(web::Json(value)),
                Err(err) => return Err(ErrorNotFound(err))
            }
        }
        None => return Err(ErrorUnauthorized("Access Denied")),
    }
}