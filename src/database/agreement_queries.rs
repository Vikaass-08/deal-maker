use crate::schema::agreement;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models;
use self::models::{NewAgreement, Agreement, AgreementList};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;

#[derive(Serialize)]
pub struct Response <T> {
  pub status: T,
  pub code: String
}

pub fn create_agreement_query(agreement_data: &str, data_type: &str) -> Result<Agreement, String> {
  use crate::schema::agreement;
  let conn = &mut establish_connection();

  let new_agreement = NewAgreement { agreement_data, data_type };

  let output = diesel::insert_into(agreement::table)
      .values(&new_agreement)
      .returning(Agreement::as_returning())
      .get_result(conn);

  match output {
      Ok(val) => Ok(val),
      Err(e) => Err(e.to_string()),
  }
}


pub fn get_agreement_query() -> Result<AgreementList, String> {
  use crate::schema::agreement::dsl::*;
  let conn = &mut establish_connection();

  let total_agreement =  agreement
      .load::<Agreement>(conn)
      .expect("Error loading agreement");

  if total_agreement.len() == 0{
    println!("\n No employees to display!! \n ");
    return Err(String::from("404"));
  }

  let results = agreement
         .order(id)
         .limit(10.into())
         .load::<Agreement>(conn)
         .expect("Error loading employees");
  return Ok(AgreementList {agreements: results});
}