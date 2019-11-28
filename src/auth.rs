//! modul s autentifikačními funkcemi

use serde::{Deserialize, Serialize};

use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;

use base64::{encode, decode};

use diesel::prelude::*;
use std::env;

use super::db::User;

/// autorizační token
#[derive(Serialize, Deserialize, )]
pub struct AuthToken {
    /// jméno uživatele
    pub name: String,
    /// email uživatele
    pub email: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let keys: Vec<_> = request.headers().get("Authorization").collect();
		match keys.get(0).unwrap_or(&"").split(' ').nth(1) {
			Some(ref token) => {
				let body = match decode(token) {
					Ok(bod) => bod,
					Err(_) => return Outcome::Failure((Status::UnprocessableEntity, "authtoken is not a correct base64 string".to_string())),
				};
				
				let token = match serde_json::from_str(&String::from_utf8_lossy(&body).to_string()) {
					Ok(tok) => tok,
					Err(e) => return Outcome::Failure((Status::UnprocessableEntity, format!("can't parse JSON into struct: {}", e.to_string()))) 
				};
				
				//... pošéfit databázi zde

                                let connection = SqliteConnection::establish(&env::var("DATABASE_URL")
                                    .expect("DATABASE_URL not in env")
                                    ).expect("error connection to db");

                                let result = users.filter(email.eq(&token.email))
                                    .first::<User>(&connection)
                                    .expect("failed to connect to db")
                                    .unwrap_or(|| {
                                        User {
                                            id: 
                                        }
                                    });

				Outcome::Success(token)
			}
			x => {
				println!("{:?}", x);
				Outcome::Failure((Status::BadRequest, "invalid authorization header".to_string()))
			}
		}
    }
}
