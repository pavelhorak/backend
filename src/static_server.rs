use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

/// servuje index
#[get("/")]
pub fn index() -> NamedFile {
	NamedFile::open("frontend/index.html").expect("index.html not found")
}

/// vrac
#[get("/static/<name..>")]
pub fn frontend(name: PathBuf) -> NamedFile {
	NamedFile::open(Path::new("frontend/build/").join(name)).expect("file not found")
}

#[catch(404)]
pub fn not_found() -> NamedFile {
	NamedFile::open("frontend/404.html").expect("404.html not found")
}
