use rocket::Route;
use rocket_contrib::json::Json;

use crate::db::{NewReservation, UpdateReservation};

/// vrací všechny rezervace
///
/// GET /events "application/json"
#[get("/events", format = "application/json")]
pub fn list() -> String {
	rgi! {
		LIST "rgi/booking/booking.py"
	}
}

/// vrátí JSON dané rezervace
///
/// GET /events/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
#[get("/events/<id>", format = "application/json")]
pub fn get(id: i32) -> String {
	rgi! {
		GET "rgi/booking/booking.py"
		arg: id
	}
}

/// vrátí JSON dané rezervace
///
/// POST /events application/json
///
/// data: [`NewReservation`]
#[post("/events", format = "application/json", data = "<_input>")]
pub fn post(_input: Json<NewReservation>) -> String {
	rgi! {
		POST "rgi/booking/booking.py"
		data: (&_input.into_inner())
	}
}

/// upraví danou rezervaci
///
/// PATCH /events/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
///
/// data:[`UpdateReservation`]
#[patch("/events/<id>", format = "application/json", data = "<_input>")]
pub fn patch(id: i32, _input: Json<UpdateReservation>) -> String {
	rgi! {
		PATCH "rgi/booking/booking.py"
		arg: id
		data: (&_input.into_inner())
	}
}

/// vymaže danou rezervaci
///
/// DELETE /events/<id> application/json
///
/// parametry:
/// - `id`: identifikátor dané rezervace
#[delete("/events/<id>", format = "application/json")]
pub fn delete(id: i32) -> String {
	rgi! {
		DELETE "rgi/booking/booking.py"
		arg: id
	}
}

///
#[get("/events/filter/<rooms>/<begin_time>/<end_time>", format = "application/json")]
pub fn date_filter(rooms: i32, begin_time: String, end_time: String) -> String {
	rgi! {
		FILTER "rgi/booking/booking.py"
		arg: rooms,
		arg: begin_time,
		arg: end_time
	}
}

/// vrací seznam endpointů pro nabindování do Rocketu
pub fn routes() -> Vec<Route> {
	routes![date_filter, list, get, post, patch, delete,]
}
