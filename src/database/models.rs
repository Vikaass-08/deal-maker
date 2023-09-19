use crate::schema::agreement;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::users;
use diesel::sql_types::*;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = agreement)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agreement {
  pub agreement_id: i32,
  pub agreement_data: String,
  pub agreement_type: String
}

#[derive(Serialize, Deserialize)]
pub struct AgreementList {
  pub agreements: Vec<Agreement>
}


#[derive(Insertable)]
#[diesel(table_name = agreement)]
pub struct NewAgreement<'a> {
    pub agreement_data: &'a str,
    pub agreement_type: &'a str,
}



#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub user_id: i32,
  pub first_name: String,
  pub last_name: String,
  pub user_type: String,
  pub user_email: String
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub user_type: &'a str,
    pub user_email: &'a str,
}