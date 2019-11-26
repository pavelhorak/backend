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
	let data = serde_json::to_string(&_input.into_inner()).unwrap();
	rgi! {
		POST "rgi/booking/booking.py"
		data: (&data)
	}
}

#[patch("/booking/<id>", format = "application/json", data = "<_input>")]
pub fn patch(id: i32, _input: Json<NewReservation>) -> String {
	let data = serde_json::to_string(&_input.into_inner()).unwrap();
	rgi! {
		PATCH "rgi/booking/booking.py"
		arg: id
		data: (&data)
	}
}

#[delete("/booking/<id>", format = "application/json")]
pub fn delete(id: i32) -> String {
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
