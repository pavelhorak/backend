#[derive(Queryable)]
pub struct Reservation {
    id: u32,
    name: String,
    description: String,
    rooms: u16,
    begin_time: String,
    end_time: String,
    layout: u16,
    approved: bool,
}
