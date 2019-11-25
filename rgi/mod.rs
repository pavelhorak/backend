use rocket::Route;

mod booking;

pub fn routes() -> Vec<Route> {
    routes![
        booking::booking,
    ]
}
