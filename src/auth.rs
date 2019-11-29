//! modul s autentifikačními funkcemi
//!
//! ## přidání autorizace k endpointu
//! stačí přidat AuthToken parametr s typem
//! ```no_run
//! #[get("/supersecretstuff")]
//! pub fn example(_u: AuthToken<roles::FacilityManager>) {
//!
//! }
//!```

use serde::{Deserialize, Serialize};

use rocket::request::{FromRequest, Request, Outcome};
use rocket_contrib::json::Json;
use rocket::http::Status;

use base64::decode;

use diesel::prelude::*;

use std::marker::PhantomData;

use crate::db;
use crate::db::{NewUser, User};

/// autorizační token, tak jak je přijat
#[derive(Serialize, Deserialize)]
pub struct AuthTokenRaw {
	/// jméno uživatele
	pub name: String,
	/// email uživatele
	pub email: String,
}

/// autorizační token po vyřešení údajů s databází
#[derive(Serialize, Deserialize)]
pub struct AuthToken<T: roles::Role> {
	/// nalezený uživatel
	pub user: User,
	/// marker pro rezoluci role
	pub _m: PhantomData<T>,
}

impl<T: roles::Role> AuthToken<T> {
	/// sestrojí nový AuthToken z instace [`User`]
	pub fn from_user(user: User) -> Self {
		AuthToken { user, _m: PhantomData }
	}
}

/// obsahuje nulové typy pro role
/// tento design umožňužje zneužít funkce Rustu pro deklarativní
/// ověření -> pouze stačí do routy přidat parametr s typem `AuthToken<Approved>`.
///
/// současné role a jejich stringy (stringy jsou case-insensitive):
/// -  [`roles::Noob`] -> `noob`
/// -  [`roles::Approver`] -> `approver`
/// -  [`roles::FacilityManager`] -> `facilitymanager`
pub mod roles {
	#![allow(dead_code, missing_docs)]

	/// sdílený trait pro role
	/// velmi rudimentárním způsobem reprezentuje hierarchii
	pub trait Role {
		/// jméno role jako string
		fn name() -> &'static str;
		/// jméno rodiše jako string
		fn daddy() -> Option<&'static str> {
			None
		}
	}

	macro_rules! role_gen {
		{$($role:ident $(-> $daddy:ident)?)*} => {
			$(
				pub struct $role;
				impl Role for $role {
					fn name() -> &'static str { stringify!($role) }
					$(fn daddy() -> Option<&'static str> { Some(stringify!($daddy)) })?
				}
			)*
		}
	 }

	role_gen! {
	   Noob
	   Approver        -> Noob
	   FacilityManager -> Noob
	}
}

impl<'a, 'r, T: roles::Role> FromRequest<'a, 'r> for AuthToken<T> {
	type Error = String;

	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		use crate::schema::users::dsl::*;

		let keys: Vec<_> = request.headers().get("Authorization").collect();
		match keys.get(0).unwrap_or(&"").split(' ').nth(1) {
			Some(ref token) => {
				eprintln!("decoding");
				let body = match decode(token) {
					Ok(bod) => bod,
					Err(_) =>
						return Outcome::Failure((
							Status::UnprocessableEntity,
							"authtoken is not a correct base64 string".to_string(),
						)),
				};

				eprintln!("parsing");
				let token: AuthTokenRaw = match serde_json::from_str(&String::from_utf8_lossy(&body).to_string()) {
					Ok(tok) => tok,
					Err(e) =>
						return Outcome::Failure((
							Status::UnprocessableEntity,
							format!("can't parse JSON into struct: {}", e.to_string()),
						)),
				};

				//... pošéfit databázi zde

				let connection = db::get_con();

				let result = users
					.filter(email.eq(&token.email))
					.first::<User>(&connection)
					.optional()
					.expect("failed to connect to db")
					.unwrap_or_else(|| {
						// unfortunately, SQLite does not support the RETURNING clause, so one has to do this atrocity
						diesel::insert_into(users)
							.values(NewUser {
								name:  token.name.clone(),
								email: token.email.clone(),
								role:  "noob".to_string(),
							})
							.execute(&connection)
							.expect("failed to connect to db or insert item");

						users
							.filter(email.eq(&token.email))
							.first::<User>(&connection)
							.unwrap_or_else(|_| unreachable!("uh oh, this shouldn't happen, is your DB okay?"))
					});

				if T::name().to_lowercase() == result.role.to_lowercase() {
					Outcome::Success(AuthToken::from_user(result))
				} else if let Some(daddy) = T::daddy() {
					println!("{}", daddy);
					match daddy.to_lowercase() == result.role.to_lowercase() {
						true => Outcome::Success(AuthToken::from_user(result)),
						false => Outcome::Failure((Status::Forbidden, "you don't have the required role".to_string())),
					}
				} else {
					Outcome::Failure((Status::Forbidden, "you don't have the required role".to_string()))
				}
			}
			x => {
				println!("{:?}", x);
				Outcome::Failure((Status::BadRequest, "invalid authorization header".to_string()))
			}
		}
	}
}

/// vrací informace o uživatelu
#[get("/me")]
pub fn me(_u: AuthToken<self::roles::Noob>) -> Json<User> {
	 Json(_u.user)
}
