//! ## backend rezervačního systému pro auditorium
//! Dokumentace backendu.
//!
//! Struktura:
//! ```bash,no_run
//! .
//! ├── build.rs    - build skript
//! ├── Cargo.lock  - lockfile, nemazat! (deterministické buildy)
//! ├── Cargo.toml  - manifest balíčku
//! ├── diesel.toml - konfigurace Diesel.rs
//! ├── Dockerfile  - dockerfile
//! ├── frontend    - submodul s frontendem
//! ├── Makefile    - make
//! ├── migrations  - migrace
//! │   ├── 00000000000000_diesel_initial_setup
//! │   │   ├── down.sql
//! │   │   └── up.sql
//! │   └── 2019-11-25-143159_reservations - migrace relace reservations
//! │       ├── down.sql
//! │       └── up.sql
//! ├── README.md   - README
//! ├── rgi         - obsahuje RGI, viz modul rgi
//! │   ├── booking - booking rgi
//! │   │   ├── booking.py
//! │   │   ├── curltest
//! │   │   └── mod.rs
//! │   └── mod.rs
//! ├── Rocket.toml  - konfigurační soubor Rocketu
//! ├── rustfmt.toml - pravidla pro automatické formátování kódu
//! └── src          - zdrojové soubory
//!     ├── db.rs            - databázové modely, utility
//!     ├── main.rs          - entrypoint programu
//!     ├── rgi -> ../rgi/   - symlink
//!     ├── schema.rs        - schéma databáze (vygenerováno Dieselem, neupravovat!)
//!     └── static_server.rs - statický server
//! ```
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
		.mount("/", routes![
                    static_server::index,
                    static_server::frontend,
                    static_server::favicon,
                ])
		.mount("/rgi/", rgi::routes())
		.attach(DbConn::fairing())
		.launch();
}
