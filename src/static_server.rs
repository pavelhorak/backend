//! statický webserver pro posílání frontendu
//!
//! posílá následující soubory skrze tyto routy:
//! - [`index`] -> __frontend/index.html__
//! - [`frontend`] -> soubory ze složky __frontend/build__
//! - [`not_found`] -> 404 soubor
//!
//! přidání nové statické routy:
//! ```rust,no_run
//! #[get("/url/<path..>")]
//! pub fn moje_route(path: PathBuf) -> NamedFile {
//!     NamedFile::open(Path::new("cesta/ke/slozce/").join(path))
//!         .expect("nepodařilo se otevřít soubor")
//! }
//! ```
//!
//! následně je zapotřebí routu zapnout v main.rs
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

/// servuje index
#[get("/")]
pub fn index() -> NamedFile {
	NamedFile::open("frontend/index.html").expect("index.html not found")
}

/// vrací statické soubory frontendu
#[get("/static/<name..>")]
pub fn frontend(name: PathBuf) -> NamedFile {
	NamedFile::open(Path::new("frontend/build/").join(name)).expect("file not found")
}

/// catcher pro 404
#[catch(404)]
pub fn not_found() -> NamedFile {
	NamedFile::open("frontend/404.html").expect("404.html not found")
}
