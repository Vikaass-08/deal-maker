use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::lender::id as lender_ag_id;
use crate::models;
use self::models::{NewDocument, Document, Lender};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;

pub fn create_document_query(doc_data: &str, doc_type: &str, len_id: &i32) -> Result<Document, String> {
  use crate::schema::document;
  use crate::schema::lender;
  let conn = &mut establish_connection();

  let lender_output = lender::table
    .filter(lender_ag_id.eq(len_id))
    .load::<Lender>(conn);

  match lender_output {
      Ok(get_lender) => {
        if get_lender.len() == 0 {
          return Err(String::from("Access denied!!"));
        }
      
        let new_document = NewDocument { document_data: doc_data, document_type: doc_type, lender_id: len_id };
      
        let output = diesel::insert_into(document::table)
            .values(&new_document)
            .returning(Document::as_returning())
            .get_result(conn);
      
        match output {
            Ok(val) => Ok(val),
            Err(e) => Err(e.to_string()),
        }
      }
      Err(e) => Err(e.to_string())
  }
}


pub fn get_document_query(document_id: i32) -> Result<Document, String> {
  use crate::schema::document::dsl::*;
  let conn = &mut establish_connection();

  let documents =  document
      .filter(id.eq(document_id))
      .first::<Document>(conn);

  match documents {
      Ok(doc_data) => Ok(doc_data),
      Err(err) => Err(err.to_string())
  }
}