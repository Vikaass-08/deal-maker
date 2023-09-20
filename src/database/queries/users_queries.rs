use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models;
use self::models::{Users, NewUsers};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;
use crate::types::CreateUserReq;

pub fn create_user_query(user_data: &CreateUserReq) -> Result<Users, String> {
	use crate::schema::users::dsl::*;
	let conn = &mut establish_connection();

	let new_user = NewUsers { 
		first_name: &user_data.first_name,
		last_name: &user_data.last_name,
		email: &user_data.email,
	};

	let user_exits = users
		.filter(email.eq(&user_data.email))
		.load::<Users>(conn)
		.expect("User already exists");

	let output = diesel::insert_into(users)
			.values(&new_user)
			.returning(Users::as_returning())
			.get_result(conn);

	match output {
			Ok(val) => Ok(val),
			Err(e) => Err(e.to_string()),
	}
}