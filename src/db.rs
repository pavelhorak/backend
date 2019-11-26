use chrono::DateTime;
use rocket_contrib::databases::diesel;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

#[derive(Queryable)]
pub struct Reservation {
    id: u16,
    name: String,
    description: String,
    author: String,
    rooms: u8,
    begin_time: DateTime,
    end_time: DateTime,
    layout: u16,
    approved: bool,
}
