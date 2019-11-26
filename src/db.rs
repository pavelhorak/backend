use rocket_contrib::databases::diesel;
use diesel::data_types::PgTime;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

#[derive(Queryable)]
pub struct Reservation {
    id: u16,
    name: String,
    description: String,
    author: String,
    rooms: u8,
    begin_time: PgTime,
    end_time: PgTime,
    layout: u16,
    approved: bool,
}
