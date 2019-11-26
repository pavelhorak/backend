use rocket::Route;
use rocket_contrib::json::Json;

use crate::db::NewReservation;


#[get("/booking", format = "application/json")]
pub fn list() -> String {
	rgi! {
		LIST "rgi/booking/booking.py"
	}
}



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
		data: (&_input.into_inner())
	}
}

#[patch("/booking/<id>", format = "application/json", data = "<_input>")]
pub fn patch(id: i32, _input: Json<NewReservation>) -> String {
	rgi! {
		PATCH "rgi/booking/booking.py"
		arg: id
		data: (&_input.into_inner())
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
                list,
		get,
		post,
		patch,
		delete
	]
}

