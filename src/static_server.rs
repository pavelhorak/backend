use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> NamedFile{
    NamedFile::open("frontend/index.html").expect("index.html not found")
}

#[get("/static/<name>")]
pub fn frontend(name: String) -> NamedFile {
    NamedFile::open(format!("frontend/build/{}", name)).expect("file not found")
}

#[catch(404)]
pub fn not_found() -> NamedFile {
    NamedFile::open("frontend/404.html").expect("404.html not found")
}
