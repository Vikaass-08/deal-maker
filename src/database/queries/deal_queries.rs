use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models;
use self::models::{Deal, NewDeal, DocumentRequest, Users, Lender, Document};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;
use crate::types::{CreateDealReq, CreateDealResp, DealStatusCode, DocStatusCode, UpdateDealReq, UpdateDealResp};
use diesel::result::Error as ResultError;


pub struct  ValidCheck {
  user_check: Result<Users, ResultError>,
  lender_check: Result<Lender, ResultError>
}

pub struct DocValidCheck {
  doc_check: Result<Document, ResultError>,
  doc_req_check: Result<DocumentRequest, ResultError>
}



pub fn create_deal_query(deal_req: &CreateDealReq) -> Result<CreateDealResp, String> {
  use crate::schema::{deal, users, lender, document_request, document};
  let conn = &mut establish_connection();

  let get_user = users::table 
        .filter(users::email.eq(&deal_req.borrower_email))
        .first::<Users>(conn);

  let get_lender = lender::table
      .filter(lender::email.eq(&deal_req.lender_email))
      .first::<Lender>(conn);

  let valid_check: ValidCheck = ValidCheck {
      user_check: get_user,
      lender_check: get_lender
  };

  match valid_check {
    ValidCheck {user_check: Ok(user_val), lender_check: Ok(lender_val) } => {
      let get_doc_s = document::table
        .filter(document::user_email.eq(&deal_req.borrower_email))
        .filter(document::lender_email.eq(&deal_req.lender_email))
        .first::<Document>(conn);

      let document_req_accepted = document_request::table
        .filter(document_request::lender_id.eq(&lender_val.id))
        .filter(document_request::user_id.eq(&user_val.id))
        .filter(document_request::status.eq(DocStatusCode::ACCEPTED.to_string()))
        .first::<DocumentRequest>(conn);

      let dock_valid_check: DocValidCheck = DocValidCheck {
          doc_check: get_doc_s,
          doc_req_check: document_req_accepted
        };
  
      match dock_valid_check {
        DocValidCheck {doc_check: Ok(valid_u), doc_req_check: Ok(valid_value) } => {
          let deal_output = deal::table
            .filter(deal::lender_id.eq(valid_value.lender_id))
            .filter(deal::user_id.eq(valid_value.user_id))
            .filter(deal::document_id.eq(valid_value.id))
            .first::<Deal>(conn);
      
          match deal_output {
              Err(diesel::result::Error::NotFound) => {
                // The Deal was not found. so create it
                let new_deal_req = NewDeal { 
                    lender_id: &lender_val.id,
                    user_id: &user_val.id,
                    document_id: &valid_u.id,
                    status: &DealStatusCode::CREATED.to_string()
                };
        
                let create_deal_request = diesel::insert_into(deal::table)
                    .values(&new_deal_req)
                    .returning(Deal::as_returning())
                    .get_result(conn);
        
                match create_deal_request {
                    Ok(val) => return Ok(
                      CreateDealResp { 
                        document_id: val.document_id, 
                        status: val.status, 
                        updated_at: val.updated_at 
                      }
                    ),
                    Err(e) => return Err(e.to_string()),
                }
              }
              Ok(deal_data) => Ok(
                CreateDealResp { document_id: deal_data.document_id, status: deal_data.status, updated_at: deal_data.updated_at }
              ),
              Err(e) => Err(e.to_string())
          }
        }
        DocValidCheck {doc_check, doc_req_check} => return Err(String::from("Unable to find Document"))
      }
    }
    ValidCheck {user_check, lender_check} => Err(String::from("lender or borrower doesn't exist"))
  }
}



pub fn update_deal_query(deal_req: &UpdateDealReq) -> Result<UpdateDealResp, String> {
  use crate::schema::{deal, document_request, users, lender};
  let conn = &mut establish_connection();

  let get_user = users::table 
        .filter(users::email.eq(&deal_req.borrower_email))
        .first::<Users>(conn);

  let get_lender = lender::table
      .filter(lender::email.eq(&deal_req.lender_email))
      .first::<Lender>(conn);

  let valid_check: ValidCheck = ValidCheck {
      user_check: get_user,
      lender_check: get_lender
  };

  match valid_check {
    ValidCheck {user_check: Ok(user_val), lender_check: Ok(lender_val) } => {

      let document_req_accepted = document_request::table
        .filter(document_request::lender_id.eq(lender_val.id))
        .filter(document_request::user_id.eq(user_val.id))
        .filter(document_request::status.eq(DocStatusCode::ACCEPTED.to_string()))
        .first::<DocumentRequest>(conn);
  
      match document_req_accepted {
        Ok(valid_value) => {
          let deal_output = deal::table
              .filter(deal::lender_id.eq(valid_value.lender_id))
              .filter(deal::user_id.eq(valid_value.user_id))
              .filter(deal::document_id.eq(valid_value.id))
              .first::<Deal>(conn);
    
          match deal_output {
            Ok(deal_data) => {
              let deal_status_val = deal_req.status.to_uppercase();
              if deal_data.status == DealStatusCode::CREATED.to_string() {
                if deal_status_val == DealStatusCode::DONE.to_string() || deal_status_val == DealStatusCode::REJECT.to_string() {
    
                  let update_query = diesel::update(deal::table)
                    .filter(deal::lender_id.eq(valid_value.lender_id))
                    .filter(deal::user_id.eq(valid_value.user_id))
                    .filter(deal::document_id.eq(valid_value.id))
                    .set(deal::status.eq(deal_status_val))
                    .get_result::<Deal>(conn);
    
                  match update_query {
                      Ok(val) => return Ok(
                          UpdateDealResp { 
                            document_id: val.document_id, 
                            status: val.status, 
                            updated_at: val.updated_at 
                          }
                      ),
                      Err(e) => return Err(e.to_string()),
                  }
                }
                else {
                  return Err(String::from("Please Enter correct status"))
                }
              }
              else {
                return Ok( UpdateDealResp { 
                  document_id: deal_data.document_id, 
                  status: deal_data.status, 
                  updated_at: deal_data.updated_at 
                })
              }
            }
            Err(e) => Err(e.to_string())
          }
        }    
        Err(err) => Err(err.to_string())
      }
      
    }
    ValidCheck {user_check, lender_check} => Err(String::from("lender or borrower doesn't exist"))
  }

}