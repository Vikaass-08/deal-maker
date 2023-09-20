use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models;
use self::models::{Lender, NewLender};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;


pub fn create_lender_query(lender_data: &Lender) -> Result<Lender, String> {
	use crate::schema::lender::dsl::*;
	let conn = &mut establish_connection();

	let new_lender = NewLender { 
		org_name: &lender_data.org_name,
		email: &lender_data.email,
	};

	let lender_exits = lender
		.filter(email.eq(&lender_data.email))
		.load::<Lender>(conn)
		.expect("Lender already exists");

	let output = diesel::insert_into(lender)
			.values(&new_lender)
			.returning(Lender::as_returning())
			.get_result(conn);

	match output {
			Ok(val) => Ok(val),
			Err(e) => Err(e.to_string()),
	}
}