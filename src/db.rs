//! obsahuje modely
//! pro přidávání nového modelu viz [dokumentace Diesel ORM](https://diesel.rs)

use rocket_contrib::databases::diesel;
use serde::{Serialize, Deserialize};

#[database("postgres_db")]
pub struct DbConn(diesel::SqliteConnection);

/// Model rezervace, tak jak je uložena v databázi
#[derive(Queryable, Debug, Clone)]
pub struct Reservation {
	/// primární klíč
	pub id: u16,
	/// název události
	pub name: String,
	/// popis události
	pub description: String,
	/// "rezervujitel" události :^)
	pub author: String,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: u8,
	/// počáteční čas rezervace
	pub begin_time:  String,
	/// čas, kdy rezervace končí
	pub end_time:    String,
	/// rozložení nábytku v audioriu
	pub layout:      u8,
	/// zda byla rezervace schválena
	pub approved:    u8,
}

/// Model rezervace pro přidání do databáze
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct NewReservation {
	/// název události
	pub name: String,
	/// popis události
	pub description: String,
	/// "rezervujitel" události :^)
	pub author: String,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: u8,
	/// počáteční čas rezervace
	pub begin_time: String,
	/// čas, kdy rezervace končí
	pub end_time: String,
	/// rozložení nábytku v audioriu
	pub layout: u16,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateReservation {
	/// název události
	pub name: Option<String>,
	/// popis události
	pub description: Option<String>,
	/// "rezervujitel" události :^)
	pub author: Option<String>,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: Option<u8>,
	/// počáteční čas rezervace
	pub begin_time: Option<String>,
	/// čas, kdy rezervace končí
	pub end_time: Option<String>,
	/// rozložení nábytku v audioriu
	pub layout: Option<u16>,
}
