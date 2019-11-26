#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

mod rgi;

mod static_server;
mod db;
mod schema;

use db::DbConn;

fn main() {
	dotenv.ok();

	rocket::ignite()
		.register(catchers![static_server::not_found])
		.mount("/", routes![static_server::index, static_server::frontend,])
		.mount("/rgi/", rgi::routes())
		.attach(DbConn::fairing())
		.launch();
}
