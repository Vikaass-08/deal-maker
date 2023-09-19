
use std::string;
use actix_web::error::ErrorNotFound;
use actix_web::{get, post, HttpResponse, Responder, web, Result, error};
use serde::{Serialize, Deserialize};

use crate::database::models::{Document, NewDocument, DocumentList};
use crate::database::document_queries::{get_document_query, create_document_query};


#[derive(Serialize, Deserialize)]
pub struct SaveDocumentReq {
    pub document_data: String,
    pub document_type: String,
    pub lender_id: i32
}

#[get("/get")]
pub async fn get_document() -> Result<impl Responder> {
    let get_query: Result<DocumentList, String> = get_document_query();
    match get_query {
        Ok(value) => return Ok(web::Json(value)),
        Err(err) => return Err(ErrorNotFound(err)),
    };
}

#[post("/create")]
pub async fn save_document(req: web::Json<SaveDocumentReq>) -> Result<impl Responder> {
  let save_agreement:Result<Document, String> = create_document_query(
        &req.document_data.to_string(), 
        &req.document_type.to_string(), 
        &req.lender_id
    );

  match save_agreement {
      Ok(value) => return Ok(web::Json(value)),
      Err(err) => return Err(ErrorNotFound(err))
  }
}