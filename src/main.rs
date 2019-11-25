#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String{
    "index".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .launch();
}
