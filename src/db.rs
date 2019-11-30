//! obsahuje modely
//! pro přidávání nového modelu viz [dokumentace Diesel ORM](https://diesel.rs)

use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;

use serde::{Serialize, Deserialize};
use sled::{Db, Tree};

use std::env;
use std::ops::Deref;
use std::sync::RwLock;
use std::marker::PhantomData;

lazy_static! {
	/// a global handle to the Sled database
	pub static ref DB: RwLock<Db> = RwLock::new({
		let db = Db::open(&env::var("DATABASE_URL").expect("failed to read DATABASE_URL environment variable"))
			.expect("failed to open database");
		db
	});
}

/// wraps the database
///
/// the reasons are two:
/// 1. to allow  FromRequest implementation
/// 2. declarative table access
pub struct Database<T: Table>(Tree, PhantomData<T>);

impl<T: Table> Database<T> {
	/// read-only access to tree
	pub fn read(&self) -> &Tree { &self.0 }
	/// read and write access to tree
	pub fn write(&mut self) -> &mut Tree { &mut self.0 }
}

/// trait for the Table marker types
pub trait Table {
	/// opening a table might not always work,
	/// this type should explain what's the issue
	type TableError = sled::Error;

	/// name (actually prefix) of the table
	fn name() -> &'static str;
	/// gets the actual tree, should do it using the global DB handle
	fn get_tree_naive() -> Result<Tree, sled::Error> {
		let lock = DB.read()
			.expect("the database rwlock has been poisoned");
			// ^ this usually implies a deeper underlying problem,
			// so it's probably okay to hard crash

		lock.open_tree(&Self::name())
	}

	/// should return true if a custom get tree function is available
	fn has_get_tree() -> bool { false }
 
	/// optional custom function for fetching a tree,
	/// can call [`Table::get_tree_naive`]
	fn get_tree() -> Result<Tree, Self::TableError> { unimplemented!()  }
}

/// module containing table markers
pub mod table {
	use super::Table;

	/// Reservation database table marker
	pub struct Reservations;

	impl Table for Reservations {
		fn name() -> &'static str { "reservation" }
	}

	/// Users database table marker
	pub struct Users;

	impl Table for Users {
		fn name() -> &'static str { "user" }
	}
}

impl<'a, 'r, T: Table> FromRequest<'a, 'r> for Database<T> {
	type Error = String;

	fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		if T::has_get_tree() {
			match T::get_tree() {
				Ok(tree) => Outcome::Success(Database(tree, PhantomData)),
				Err(_) => Outcome::Failure((Status::InternalServerError, "failed to run custom database function and acquire tree".to_string())),
			}
		} else {
			match T::get_tree_naive() {
				Ok(tree) => Outcome::Success(Database(tree, PhantomData)),
				Err(_) => Outcome::Failure((Status::InternalServerError, "failed to acquire tree".to_string())),
			}
		}
	}
}

