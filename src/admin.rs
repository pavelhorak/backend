//! a module containing admin functions
use rocket::Route;
use rocket_contrib::json::Json;

use std::env;

use crate::auth::AuthToken;
use crate::auth::roles::{Role, Superadmin};

use crate::db::{
	Database,
	table::Reservations,
	table::Users,
};

use crate::models::{User, Reservation};

/// geenrates a superadmin
#[post("/generate_sa/<email>/<password>")]
pub fn generate_superadmin(email: String, password: String, mut db: Database<Users>) -> Option<()> {
	if password != env::var("SA_SECRET").unwrap() { return None }

	db.write()
		.insert(&email, User { email: email.clone(), name: "Superadmin".to_string(), role: Superadmin::name().to_string() })
		.ok()?
		.map(|_| ())
}

/// get all users
#[get("/users", format = "application/json")]
pub fn users(db: Database<Users>, _u: AuthToken<Superadmin>) -> Json<Vec<(String, User)>> {
	Json(db.read().iter().collect::<Vec<(String, User)>>())
}


/// vrací seznam endpointů pro nabindování do Rocketu
pub fn routes() -> Vec<Route> {
	routes![users, generate_superadmin]
}
