use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> String{
    "index".to_string()
}
