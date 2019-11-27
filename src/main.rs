//! ## backend rezervačního systému pro auditorium
//!
//!
#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_docs)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

pub mod rgi;
pub mod db;
pub mod static_server;

/// schéma databáze (vygenerováno Dieselem)
#[allow(missing_docs)]
pub mod schema;

use db::DbConn;

fn main() {
	dotenv().ok();

	rocket::ignite()
		.register(catchers![static_server::not_found])
		.mount("/", routes![static_server::index, static_server::frontend,])
		.mount("/rgi/", rgi::routes())
		.attach(DbConn::fairing())
		.launch();
}
