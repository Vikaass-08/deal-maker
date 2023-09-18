use crate::schema::agreement;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = agreement)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agreement {
  pub id: i32,
  pub agreement_data: String,
  pub data_type: String
}

#[derive(Serialize, Deserialize)]
pub struct AgreementList {
  pub agreements: Vec<Agreement>
}


#[derive(Insertable)]
#[diesel(table_name = agreement)]
pub struct NewAgreement<'a> {
    pub agreement_data: &'a str,
    pub data_type: &'a str,
}
