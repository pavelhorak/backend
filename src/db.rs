use rocket_contrib::databases::diesel;
use diesel::data_types::PgTime;
use serde::{Serialize, Deserialize};

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

#[derive(Queryable, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct NewReservation {
    name: String,
    description: String,
    author: String,
    rooms: u8,
    begin_time: String,
    end_time: String,
    layout: u16,
    approved: bool,
}
