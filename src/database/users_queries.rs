use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models;
use self::models::{User, NewUser};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;

pub fn create_user_query(user_data: &User) -> Result<User, String> {
	use crate::schema::users::dsl::*;
	let conn = &mut establish_connection();

	let new_user = NewUser { 
			first_name: &user_data.first_name,
			last_name: &user_data.last_name,
			user_type: &user_data.user_type,
			user_email: &user_data.user_email,
	};

	let user_exits = users
			.filter(user_email.eq(&user_data.user_email))
			.load::<User>(conn)
			.expect("User already exists");

	let output = diesel::insert_into(users)
			.values(&new_user)
			.returning(User::as_returning())
			.get_result(conn);

	match output {
			Ok(val) => Ok(val),
			Err(e) => Err(e.to_string()),
	}
}