use crate::schema::lender;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models;
use self::models::{Lender, NewLender};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;
use crate::types::{CreateLenderReq, LoginLenderReq, LoginLenderResp, TokenClaims};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use dotenv::dotenv;
use std::env;


pub fn create_lender_query(lender_data: &CreateLenderReq) -> Result<Lender, String> {
	use crate::schema::lender::dsl::*;
	let conn = &mut establish_connection();

	let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash_password = hasher
        .with_password(&lender_data.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

	let new_lender = NewLender { 
		org_name: &lender_data.org_name,
		email: &lender_data.email,
		password: &hash_password
	};

	let lender_outpt = lender
		.filter(email.eq(&lender_data.email))
		.load::<Lender>(conn);

	match lender_outpt {
			Ok(lender_exits) => {
				if lender_exits.len() != 0 {
						return Err(String::from("Lender Already Exist!!"));
				}
			
				let output = diesel::insert_into(lender)
						.values(&new_lender)
						.returning(Lender::as_returning())
						.get_result(conn);
			
				match output {
						Ok(val) => Ok(val),
						Err(e) => Err(e.to_string()),
				}
			}
			Err(err) => Err(err.to_string()),
	}
}



pub fn login_lender_query(creds: &LoginLenderReq) -> Result<LoginLenderResp, String> {
	dotenv().ok();
	let conn = &mut establish_connection();

	let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
			env::var("JWT_SECRET_LENDER")
				.expect("JWT_SECRET_LENDER must be set!")
				.as_bytes(),
		)
		.unwrap();

    let email_data = &creds.lender_email;
    let password_data = &creds.lender_password;

	let output_lender = lender::table
        .filter(lender::email.eq(&email_data))
        .first::<Lender>(conn);

	match output_lender {
			Ok(get_lender) => {
				let hash_secret = env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
				let mut verifier = Verifier::default();
				let is_valid = verifier
					.with_hash(get_lender.password)
					.with_password(password_data)
					.with_secret_key(hash_secret)
					.verify()
					.unwrap();

				if is_valid {
					let claims = TokenClaims {
						id: get_lender.id
					};
					let token_str = claims.sign_with_key(&jwt_secret).unwrap();
					return Ok(LoginLenderResp {jwt_token: token_str});
				} else {
					return Err(String::from("Incorrect email or password"));
				}
			}
			Err(err) => Err(String::from("Incorrect email or password"))
	}

}