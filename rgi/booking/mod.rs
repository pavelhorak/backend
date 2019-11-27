use rocket::Route;
use rocket_contrib::json::Json;

use crate::db::NewReservation;

/// vrací všechny rezervace
///
/// GET /booking "application/json"
#[get("/booking", format = "application/json")]
pub fn list() -> String {
	rgi! {
		LIST "rgi/booking/booking.py"
	}
}

/// vrátí JSON dané rezervace
///
/// GET /booking/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
#[get("/booking/<id>", format = "application/json")]
pub fn get(id: i32) -> String {
	rgi! {
		GET "rgi/booking/booking.py"
		arg: id
	}
}

/// vrátí JSON dané rezervace
///
/// POST /booking application/json
///
/// data: [`NewReservation`]
#[post("/booking", format = "application/json", data = "<_input>")]
pub fn post(_input: Json<NewReservation>) -> String {
	rgi! {
		POST "rgi/booking/booking.py"
		data: (&_input.into_inner())
	}
}

/// upraví danou rezervaci
///
/// PATCH /booking/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
///
/// data:[`NewReservation`]
#[patch("/booking/<id>", format = "application/json", data = "<_input>")]
pub fn patch(id: i32, _input: Json<NewReservation>) -> String {
	rgi! {
		PATCH "rgi/booking/booking.py"
		arg: id
		data: (&_input.into_inner())
	}
}

/// vymaže danou rezervaci
///
/// DELETE /booking/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
#[delete("/booking/<id>", format = "application/json")]
pub fn delete(id: i32) -> String {
	rgi! {
		DELETE "rgi/booking/booking.py"
		arg: id
	}
}

/// vrací seznam endpointů pro nabindování do Rocketu
pub fn routes() -> Vec<Route> {
	routes![
		list,
		get,
		post,
		patch,
		delete,
	]
}

