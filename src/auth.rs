use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use rocket::http::Status;

pub struct Userman {
    id: u16,
    name: String,
    email: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Userman {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        unimplemented!();
    }
}
