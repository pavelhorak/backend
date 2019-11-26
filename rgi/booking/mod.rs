use rocket::Route;
use rocket_contrib::json::Json;

use crate::db::NewReservation;

#[get("/booking/<id>", format = "application/json")]
pub fn get(id: i32) -> String {
	rgi! {
		GET "rgi/booking/booking.py"
		arg: id
	}
}

#[post("/booking", format = "application/json", data = "<_input>")]
pub fn post(_input: Json<NewReservation>) -> String {
	rgi! {
		POST "rgi/booking/booking.py"
		data: serde_json::to_string(_input).unwrap()
	}
}

#[patch("/booking/<id>", format = "application/json", data = "<_input>")]
pub fn post(id: i32, _input: Json<NewReservation>) -> String {
	rgi! {
		PATCH "rgi/booking/booking.py"
		arg: id
		data: serde_json::to_string(_input).unwrap()
	}
}

#[delete("/booking/<id>", format = "application/json")]
pub fn get(id: i32) -> String {
	rgi! {
		DELETE "rgi/booking/booking.py"
		arg: id
	}
}

pub fn routes() -> Vec<Route> {
	routes![
		get,
		post,
		patch,
		delete
	]
} 
