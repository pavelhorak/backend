#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate chrono;

mod rgi;

mod static_server;
mod db;
mod models;
mod schema;

use db::DbConn;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            static_server::index,
            static_server::frontend,
        ])
        .mount("/rgi/", rgi::routes())
        .attach(
            DbConn::fairing()
        )
        .launch();
}
