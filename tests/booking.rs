extern crate rocket;
extern crate backend;
use rocket::local::Client;


#[test]
pub fn test_booking() {
	let cli = Client::new(backend::init()).expect("wtf? the virtual client failed");
	assert_eq!(cli.get("/rgi/events/").dispatch().body_string().unwrap(), "[]");
}