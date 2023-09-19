use crate::schema::agreement;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models;
use self::models::{NewAgreement, Agreement, AgreementList};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;

pub fn create_agreement_query(agree_data: &str, agree_type: &str) -> Result<Agreement, String> {
  use crate::schema::agreement::dsl::*;
  let conn = &mut establish_connection();

  let new_agreement = NewAgreement { agreement_data: agree_data, agreement_type: agree_type };

  let output = diesel::insert_into(agreement)
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
    println!("\n No agreement to display!! \n ");
    return Err(String::from("404"));
  }

  let results = agreement
         .order(agreement_id)
         .limit(10.into())
         .load::<Agreement>(conn)
         .expect("Error loading agreements");
  return Ok(AgreementList {agreements: results});
}