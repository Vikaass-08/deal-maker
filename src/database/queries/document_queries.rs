use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::{schema::lender::id as lender_ag_id, database::schema::document::lender_email};
use crate::models;
use self::models::{NewDocument, Document, Lender, Users};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;
use diesel::result::Error as ResultError;

pub struct  ValidCheck {
  user_check: Result<Users, ResultError>,
  lender_check: Result<Lender, ResultError>
}

pub fn create_document_query(doc_data: &str, doc_type: &str, lend_email: &str, user_email: &str) -> Result<Document, String> {
  use crate::schema::{document, lender, users};
  let conn = &mut establish_connection();

  let get_user = users::table 
        .filter(users::email.eq(user_email))
        .first::<Users>(conn);

  let get_lender = lender::table
      .filter(lender::email.eq(lend_email))
      .first::<Lender>(conn);

  let valid_check: ValidCheck = ValidCheck {
      user_check: get_user,
      lender_check: get_lender
  };

  match valid_check {
    ValidCheck {user_check: Ok(user_val), lender_check: Ok(lender_val) } => {
        let new_document = NewDocument { 
          document_data: &doc_data, 
          document_type: &doc_type, 
          lender_email: &lend_email,
          user_email: &user_email 
        };
      
        let output = diesel::insert_into(document::table)
            .values(&new_document)
            .returning(Document::as_returning())
            .get_result(conn);
      
        match output {
            Ok(val) => Ok(val),
            Err(e) => Err(e.to_string()),
        }
      }
      ValidCheck {user_check, lender_check} => Err(String::from("lender or borrower doesn't exist"))
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