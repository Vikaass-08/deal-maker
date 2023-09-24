use crate::schema::{users, lender, document, deal, document_request};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use diesel::sql_types::*;
use chrono::NaiveDateTime;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = document)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
  pub id: i32,
  pub lender_email: String,
  pub user_email: String,
  pub document_type: String,
  pub document_data: String,
  pub updated_at: NaiveDateTime
}


#[derive(Insertable)]
#[diesel(table_name = document)]
pub struct NewDocument<'a> {
    pub document_data: &'a str,
    pub document_type: &'a str,
    pub lender_email: &'a str,
    pub user_email: &'a str
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password: String,
  pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUsers<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// 

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = lender)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Lender {
  pub id: i32,
  pub org_name: String,
  pub email: String,
  pub password: String,
  pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = lender)]
pub struct NewLender<'a> {
    pub org_name: &'a str,
    pub email: &'a str,
    pub password: &'a str
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// 

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = deal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Deal {
  pub id: i32,
  pub lender_id: i32,
  pub user_id: i32,
  pub document_id: i32,
  pub status: String,
  pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = deal)]
pub struct NewDeal<'a> {
    pub lender_id: &'a i32,
    pub user_id: &'a i32,
    pub document_id: &'a i32,
    pub status: &'a str,
}



////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// 

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = document_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DocumentRequest {
  pub id: i32,
  pub lender_id: i32,
  pub user_id: i32,
  pub status: String,
  pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = document_request)]
pub struct NewDocumentRequest<'a> {
    pub lender_id: &'a i32,
    pub user_id: &'a i32,
    pub status: &'a str,
}