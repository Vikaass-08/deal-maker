use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::{schema::lender::id as lender_ag_id, database::schema::document::lender_email};
use crate::models;
use self::models::{NewDocument, Document, Lender, Users, DocumentRequest, Deal};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;
use diesel::result::Error as ResultError;
use crate::types::DocStatusCode;

pub struct  ValidCheck <T, U> {
  first_check: Result<T, ResultError>,
  second_check: Result<U, ResultError>
}

pub fn create_document_query(doc_data: &str, doc_type: &str, lend_email: &str, user_email: &str) -> Result<Document, String> {
  use crate::schema::{document, lender, users, document_request, deal};
  let conn = &mut establish_connection();

  let get_user = users::table 
        .filter(users::email.eq(user_email))
        .first::<Users>(conn);

  let get_lender = lender::table
      .filter(lender::email.eq(lend_email))
      .first::<Lender>(conn);

  let valid_check: ValidCheck <Users, Lender> = ValidCheck {
    first_check: get_user,
    second_check: get_lender
  };

  match valid_check{
    ValidCheck {first_check: Ok(user_val), second_check: Ok(lender_val) } => {

        let get_doc_req_values = document_request::table
            .filter(document_request::user_id.eq(user_val.id))
            .filter(document_request::lender_id.eq(lender_val.id))
            .filter(document_request::status.eq(DocStatusCode::ACCEPTED.to_string()))
            .first::<DocumentRequest>(conn);
        
        let doc_exists = document::table
            .filter(document::lender_email.eq(lender_val.email))
            .filter(document::user_email.eq(user_val.email))
            .first::<Document>(conn);

        let doc_checks: ValidCheck <DocumentRequest, Document> = ValidCheck {
          first_check: get_doc_req_values,
          second_check: doc_exists
        };

        match doc_checks {
          ValidCheck {first_check: Ok(dock_req_val), second_check: Err(diesel::result::Error::NotFound) } => {

            let deal_output = deal::table
              .filter(deal::lender_id.eq(lender_val.id))
              .filter(deal::user_id.eq(user_val.id))
              .first::<Deal>(conn);

            match deal_output {
                Err(diesel::result::Error::NotFound) => {
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
                },
                Ok(deal_status) => return Err(String::from("deal has already been started/completed")),
                Err(err) => return Err(err.to_string())
            }

          }
          ValidCheck {first_check, second_check} => return Err(String::from("document request has not been accepted or document already created"))
        }
        
      }
      ValidCheck {first_check, second_check} => Err(String::from("lender or borrower doesn't exist"))
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