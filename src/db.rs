use rocket_contrib::databases::diesel;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);
