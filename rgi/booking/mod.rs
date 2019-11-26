use rocket::Route;

#[get("/booking/<id>", format = "application/json")]
pub fn get(id: i32) -> String {
    rgi! {
		GET "rgi/booking/booking.py"
		arg: id
    }
}

pub fn routes() -> Vec<Route> {
	routes![
		get,
	]
} 
