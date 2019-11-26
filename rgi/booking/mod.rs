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
	 unimplemented!()
	 /* rgi! { ... data: serde_json::to_string(_input).unwrap() } */
}

pub fn routes() -> Vec<Route> {
	routes![
		get,
		post,
	]
} 
