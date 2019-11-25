#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod static_server;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            static_server::index,
        ])
        .launch();
}
