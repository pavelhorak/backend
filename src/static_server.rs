use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> String{
    "index".to_string()
}

#[get("/<name>")]
pub fn frontend(name: String) -> NamedFile {
    NamedFile::open(format!("frontend/build/{}", name)).expect("file not found")
}

#[get("/rgi/<name>")]
pub fn rgi(name: String) -> String {
    "wip".to_string()
}

#[catch(404)]
pub fn not_found() -> NamedFile {
    NamedFile::open("fontend/404.html").expect("404.html not found")
}
