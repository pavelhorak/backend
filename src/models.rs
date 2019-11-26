#[derive(Queryable)]
pub struct Reservation {
    id: u16,
    name: String,
    description: String,
    author: String,
    rooms: u8,
    begin_time: String,
    end_time: String,
    layout: u16,
    approved: bool,
}
