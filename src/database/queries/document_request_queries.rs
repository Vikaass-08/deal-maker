use diesel::expression::ValidGrouping;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::database::models::{Users, Lender, DocumentRequest, NewDocumentRequest};
use crate::database::schema::deal::lender_id;
use crate::types::{DocumentRequestUsersAPIReq, DocumentRequestAPIResp, DocStatusCode, DocumentRequestLenderAPIReq};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;
use diesel::result::Error as ResultError;


pub struct  ValidCheck {
    user_check: Result<Users, ResultError>,
    lender_check: Result<Lender, ResultError>
}



pub fn document_request_status_query_users(doc_req: &DocumentRequestUsersAPIReq) -> Result<DocumentRequestAPIResp, String> {
    use crate::schema::{document_request, lender, users};
	let conn = &mut establish_connection();

    let get_user = users::table 
        .filter(users::email.eq(&doc_req.borrower_email))
        .first::<Users>(conn);

    let get_lender = lender::table
        .filter(lender::email.eq(&doc_req.lender_email))
        .first::<Lender>(conn);

    let valid_check = ValidCheck {
        user_check: get_user,
        lender_check: get_lender
    };

    match valid_check {
        ValidCheck {user_check: Ok(user_val), lender_check: Ok(lender_val) } => {
            let get_doc_values = document_request::table
                .filter(document_request::user_id.eq(user_val.id))
                .filter(document_request::lender_id.eq(lender_val.id))
                .first::<DocumentRequest>(conn);

            match get_doc_values {
                Err(diesel::result::Error::NotFound) => {
                    // The DocumentRequest was not found. so create it
                    let new_doc_req = NewDocumentRequest { 
                        lender_id: &lender_val.id,
                        user_id: &user_val.id,
                        status: &DocStatusCode::INITITATED.to_string()
                    };

                    let create_document_request = diesel::insert_into(document_request::table)
                        .values(&new_doc_req)
                        .returning(DocumentRequest::as_returning())
                        .get_result(conn);

                    match create_document_request {
                        Ok(val) => return Ok(
                            DocumentRequestAPIResp { 
                                lender_email: lender_val.email, 
                                borrower_email: user_val.email, 
                                request_status: val.status, 
                                created_at: val.updated_at 
                            }
                        ),
                        Err(e) => return Err(String::from("Unable to create document request")),
                    }
                }
                Ok(doc_status) => {
                    return Ok(
                        DocumentRequestAPIResp {
                            lender_email: user_val.email,
                            borrower_email: lender_val.email,
                            request_status: doc_status.status,
                            created_at: doc_status.updated_at
                        }
                    )
                },
                Err(err) => return Err(String::from("Error in Document request"))
            }

            return Err(String::from("lender or borrower doesn't exist"));
        }
        ValidCheck {user_check, lender_check} => Err(String::from("lender or borrower doesn't exist"))
    }
}







pub fn document_request_status_query_lender(update_doc_req_status: bool, doc_req: &DocumentRequestLenderAPIReq) -> Result<DocumentRequestAPIResp, String> {
    use crate::schema::{document_request, lender, users};
	let conn = &mut establish_connection();

    let get_user = users::table 
        .filter(users::email.eq(&doc_req.borrower_email))
        .first::<Users>(conn);

    let get_lender = lender::table
        .filter(lender::email.eq(&doc_req.lender_email))
        .first::<Lender>(conn);

    let valid_check = ValidCheck {
        user_check: get_user,
        lender_check: get_lender
    };

    match valid_check {
        ValidCheck {user_check: Ok(user_val), lender_check: Ok(lender_val) } => {
            let get_doc_values = document_request::table
                .filter(document_request::user_id.eq(user_val.id))
                .filter(document_request::lender_id.eq(lender_val.id))
                .first::<DocumentRequest>(conn);

            match get_doc_values {
                Ok(doc_status) => {
                    if update_doc_req_status && doc_status.status == DocStatusCode::INITITATED.to_string() {    
                        let update_query = diesel::update(document_request::table)
                            .filter(document_request::lender_id.eq(lender_val.id))
                            .set(document_request::status.eq(&doc_req.status.to_uppercase()))
                            .get_result::<DocumentRequest>(conn);

                        match update_query {
                            Ok(val) => return Ok(
                                DocumentRequestAPIResp { 
                                    lender_email: lender_val.email, 
                                    borrower_email: user_val.email, 
                                    request_status: val.status, 
                                    created_at: val.updated_at 
                                }
                            ),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    else {
                        return Ok(
                            DocumentRequestAPIResp { 
                                lender_email: lender_val.email, 
                                borrower_email: user_val.email, 
                                request_status: doc_status.status, 
                                created_at: doc_status.updated_at 
                            }
                        )
                    }

                },
                Err(err) => return Err(String::from("Lender of Borrower doesn't exist"))
            }

            return Err(String::from("lender or borrower doesn't exist"));
        }
        ValidCheck {user_check, lender_check} => Err(String::from("lender or borrower doesn't exist"))
    }
}

