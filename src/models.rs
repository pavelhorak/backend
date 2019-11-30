//! contains database models and helper structs

use serde::{Serialize, Deserialize};
use auth::AuthToken;

/// Model rezervace, tak jak je uložena v databázi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
	/// primární klíč
	pub id: u64,
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
	pub layout: u8,
	/// zda byla rezervace schválena
	pub approved: bool,
	/// počet lidí
	pub people: i8,
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
	pub layout: u8,
	/// počet lidí
	pub people: u8,
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
	pub layout: Option<u8>,
	/// počet lidí
	pub people: Option<u8>,
}

/// Model usera
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct User {
	/// identifikátor
	pub id: u64,
	/// jméno uživatele
	pub name: String,
	/// email
	pub email: String,
	/// role
	pub role: AuthToken,
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
	pub role: AuthToken,
}
