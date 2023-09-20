use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models;
use self::models::{Users, NewUsers};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;
use crate::types::{CreateUserReq, LoginUserReq, TokenClaims, LoginUserResp};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use dotenv::dotenv;
use std::env;


pub fn create_user_query(user_data: &CreateUserReq) -> Result<Users, String> {
	use crate::schema::users::dsl::*;
	let conn = &mut establish_connection();

	let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash_password = hasher
        .with_password(&user_data.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();


	let new_user = NewUsers { 
		first_name: &user_data.first_name,
		last_name: &user_data.last_name,
		email: &user_data.email,
		password: &hash_password
	};

	let user_exits = users
		.filter(email.eq(&user_data.email))
		.load::<Users>(conn);

	match user_exits {
		Ok(user_check) => {
			if user_check.len() != 0 {
				return Err(String::from("User Already Exist!!"));
			}
		
			let output = diesel::insert_into(users)
					.values(&new_user)
					.returning(Users::as_returning())
					.get_result(conn);
		
			match output {
					Ok(val) => Ok(val),
					Err(e) => Err(e.to_string()),
			}
		}
		Err(e) => Err(e.to_string()),
	}
}


pub fn login_user_query(creds: &LoginUserReq) -> Result<LoginUserResp, String> {
	dotenv().ok();
	let conn = &mut establish_connection();

	let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
			env::var("JWT_SECRET_USER")
				.expect("JWT_SECRET_USER must be set!")
				.as_bytes(),
		)
		.unwrap();

    let email_data = &creds.user_email;
    let password_data = &creds.user_password;

	let user_output = users::table
        .filter(users::email.eq(&email_data))
        .first::<Users>(conn);

	match user_output {
		Ok(get_user) => {
			let hash_secret = env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
			let mut verifier = Verifier::default();
			let is_valid = verifier
				.with_hash(get_user.password)
				.with_password(password_data)
				.with_secret_key(hash_secret)
				.verify()
				.unwrap();

			if is_valid {
				let claims = TokenClaims {
					id: get_user.id
				};
				let token_str = claims.sign_with_key(&jwt_secret).unwrap();
				return Ok(LoginUserResp {jwt_token: token_str});
			} else {
				return Err(String::from("Incorrect username or password"));
			}
		}
		Err(e) => Err(e.to_string())
	}

}