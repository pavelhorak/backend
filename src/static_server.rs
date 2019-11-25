use rocket::response::NamedFile;

#[get("/")]
fn index() -> String{
    "index".to_string()
}
