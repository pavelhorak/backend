//! kořenový rgi modul (endpointy jsou tady, yee-haw)
//!
//! _"je to jako CGI, ale s Rustem"_
//!
//!

use rocket::Route;

/// modul obsahující endpointy pro CRUD na rezervaci
pub mod booking;

/// sbírá jednotlivé endpointy
///
/// přidání nového rgi
/// ```no_run
/// // deklarace modulu
/// mod muj_modul;
///
/// // v routes..
/// routes.extend(self::muj_modul::routes());
/// ```
pub fn routes() -> Vec<Route> {
	let mut routes = vec![];

	/*routes() funkce volat tady*/
	routes.extend(self::booking::routes());

	routes
}
