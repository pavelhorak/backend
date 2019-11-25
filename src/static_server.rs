use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> String{
    "index".to_string()
}

#[get("/<name>")]
pub fn frontend(name: String) -> NamedFile {
    NamedFile::open(format!("frontend/build/{}", name)).expect("file not found")
}

#[catch(404)]
pub not_found() -> NamedFile {
    NamedFile::open("fontend/404.html").expect("404.html not found")
}
