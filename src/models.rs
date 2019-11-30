//! contains database models and helper structs

use serde::{Serialize, Deserialize};
use crate::auth::{AuthToken, roles::Role};

/// TYP ID!!!
pub type ID = u64;
/// DEFAULTNI CISELNY TYP!!!
pub type NUMBER = u8;
/// DEFAULTNI TEXTOVY TYP!!!
pub type TEXT = String;

/// Model rezervace, tak jak je uložena v databázi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
	/// primární klíč
	pub id: ID,
	/// název události
	pub name: TEXT,
	/// popis události
	pub description: TEXT,
	/// "rezervujitel" události :^)
	pub author: TEXT,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: NUMBER,
	/// počáteční čas rezervace
	pub begin_time: TEXT,
	/// čas, kdy rezervace končí
	pub end_time: TEXT,
	/// rozložení nábytku v audioriu
	pub layout: NUMBER,
	/// zda byla rezervace schválena
	pub approved: bool,
	/// počet lidí
	pub people: NUMBER,
}

/// Model rezervace pro přidání do databáze
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct NewReservation {
	/// název události
	pub name: TEXT,
	/// popis události
	pub description: TEXT,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: NUMBER,
	/// počáteční čas rezervace
	pub begin_time: TEXT,
	/// čas, kdy rezervace končí
	pub end_time: TEXT,
	/// rozložení nábytku v audioriu
	pub layout: NUMBER,
	/// počet lidí
	pub people: NUMBER,
}

/// Weird quick models
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateReservation {
	/// název události
	pub name: Option<TEXT>,
	/// popis události
	pub description: Option<TEXT>,
	/// místnosti, které si "rezervujitel" přeje zarezervovat
	///
	/// funguje na bázi bitflagů:
	/// ```
	/// 0b00 -> žádná místnosti (nemělo by se stát :D)
	/// 0b01 -> north
	/// 0b10 -> south
	/// 0b11 -> celé auditorium
	/// ```
	pub rooms: Option<NUMBER>,
	/// počáteční čas rezervace
	pub begin_time: Option<TEXT>,
	/// čas, kdy rezervace končí
	pub end_time: Option<TEXT>,
	/// rozložení nábytku v audioriu
	pub layout: Option<NUMBER>,
	/// počet lidí
	pub people: Option<NUMBER>,
}

/// Model usera
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct User {
	/// identifikátor
	pub id: ID,
	/// jméno uživatele
	pub name: TEXT,
	/// email
	pub email: TEXT,
	/// role
	pub role: TEXT,
}

/// Model usera pro vložení do databáze
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct NewUser {
	/// jmeno
	pub name: TEXT,
	/// email
	pub email: TEXT,
	/// role
	pub role: TEXT,
}
