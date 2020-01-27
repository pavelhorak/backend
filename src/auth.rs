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

use std::marker::PhantomData;

use crate::db::{
	Database,
	table::Users,
};
use crate::models::User;

/// autorizační token, tak jak je přijat
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthTokenRaw {
	/// jméno uživatele
	pub name: String,
	/// email uživatele
	pub email: String,
}

impl AuthTokenRaw {
	/// converts a raw AuthToken into user
	pub fn into_user(self) -> User {
		use self::roles::Role;

		User {
			name: self.name,
			email: self.email,
			role: roles::Noob::name().to_string(),
		}
	}
}

/// autorizační token po vyřešení údajů s databází
#[derive(Serialize, Deserialize, Debug, Clone)]
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

	/// common trait for roles
	/// comes with basic compile-time hierarchy
	pub trait Role {
		/// this role's daddy, can be set to self if role is root
		type Daddy: Role;

		/// name of role as str
		fn name() -> &'static str;

		/// whether the role is root role
		fn is_root() -> bool { false }

		/// jméno rodiče jako string
		fn resolve_daddy() -> Vec<&'static str> {
			if Self::is_root() {
				vec![]
			} else {
				let mut v = vec![Self::name()];
				v.append(&mut Self::Daddy::resolve_daddy());
				v
			}
		}
	}

	macro_rules! role_gen {
		{$($role:ident [$daddy:ident] $(-> $is_root:literal)? ),* $(,)?} => {
			$(
				pub struct $role;
				impl Role for $role {
					type Daddy = $daddy;
					fn name() -> &'static str { stringify!($role) }
					$(fn is_root() -> bool { $is_root })?
				}
			)*
		}
	 }

	role_gen! {
		Noob[Noob]            -> true,
		Approver[Noob]        -> false,
		FacilityManager[Noob] -> false,
		Superadmin[Approver]  -> false,
	}
}

impl<'a, 'r, T: roles::Role> FromRequest<'a, 'r> for AuthToken<T> {
	type Error = String;

	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let mut db = match Database::<Users>::open() {
			Some(d) => d,
			None => return Outcome::Failure((Status::InternalServerError, "failed to connect to db".to_string())),
		};

		let keys: Vec<_> = request.headers().get("Authorization").collect();
		match keys.get(0).unwrap_or(&"").split(' ').nth(1) {
			Some(ref token) => {
				let body = match decode(token) {
					Ok(bod) => bod,
					Err(_) =>
						return Outcome::Failure((
							Status::UnprocessableEntity,
							"authtoken is not a correct base64 string".to_string(),
						)),
				};

				let token: AuthTokenRaw = match serde_json::from_str(&String::from_utf8_lossy(&body).to_string()) {
					Ok(tok) => tok,
					Err(e) =>
						return Outcome::Failure((
							Status::UnprocessableEntity,
							format!("can't parse JSON into struct: {}", e.to_string()),
						)),
				};

				//... pošéfit databázi zde

				let result = if let Some((_, u)) = db.read()
					.iter()
					.find(|(_, u)| u.email == token.email)
				{ u } else {
					let new_u = token.clone().into_user();

					if db.write()
						.insert(token.email, &new_u)
						.is_err()
					{
						return Outcome::Failure((
							Status::InternalServerError,
							"failed to insert user into DB".to_string(),
						))
					}

					new_u
				};

				if T::name() == result.role || T::resolve_daddy().contains(&result.role.as_str()) {
					Outcome::Success(AuthToken::from_user(result))
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
