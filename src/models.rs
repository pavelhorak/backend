//! contains database models and helper structs

use serde::{Serialize, Deserialize};

/// Model rezervace, tak jak je uložena v databázi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
	/// primární klíč
	pub id: i32,
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
	pub rooms: i32,
	/// počáteční čas rezervace
	pub begin_time: String,
	/// čas, kdy rezervace končí
	pub end_time: String,
	/// rozložení nábytku v audioriu
	pub layout: i32,
	/// zda byla rezervace schválena
	pub approved: i32,
	/// počet lidí
	pub people: i32,
}

/// Model rezervace pro přidání do databáze
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct NewReservation {
	/// název události
	pub name: String,
	/// popis události
	pub description: String,
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
	/// počet lidí
	pub people: u16,
}

/// Weird quick models
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateReservation {
	/// název události
	pub name: Option<String>,
	/// popis události
	pub description: Option<String>,
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
	/// počet lidí
	pub people: Option<u16>,
}

/// Model usera
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct User {
	/// identifikátor
	pub id: i32,
	/// jméno uživatele
	pub name: String,
	/// email
	pub email: String,
	/// role
	pub role: String,
}

/// Model usera pro vložení do databáze
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct NewUser {
	/// jmeno
	pub name: String,
	/// email
	pub email: String,
	/// role
	pub role: String,
}
