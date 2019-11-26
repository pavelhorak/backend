//! kořenový rgi modul (endpointy jsou tady, yee-haw)
//!
//! _"je to jako CGI, ale s Rustem"_
//!
//!

use rocket::Route;

/// makro, které vygeneruje boilerplate pro volání daného rgi
///
/// syntaxe (hranaté závorky značí, že parametr není povinný):
/// ```rust
/// rgi! {
///     HTTP_METODA "rgi/cesta/k/rgi/binarce"
///     [arg: identifikátor]* // argumentem se myslí parametr z URL
///     [data: <vyraz>]? // někdy je potřeba obalit do závorek ()
/// }
/// ```
/// (všechny argumenty a data musí implementovat `serialize` ze `serde`)
///
/// příklad:
/// rgi! {
///     GET "rgi/lol/lol.py"
///     arg: name
///     arg: password
///     data: (Objekt)
/// }
#[macro_export]
macro_rules! rgi {
	{$method:ident $name:literal $(arg: $arg:ident),*  $(data: $data:tt)? } => {
		#[allow(unused_imports)]
		use std::process::{Command, Stdio};
		#[allow(unused_imports)]
		use std::io::{Read, Write};

		let mut cmd = Command::new($name)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.arg(stringify!($method))
			.spawn()
			.expect("kinda gay");

		if let Some(ref mut stdin) = &mut cmd.stdin {
			let _ = writeln!(stdin, "{{");
			let _ = writeln!(stdin, "\t\"args\": {{");
			$(let _ = writeln!(stdin, "\t\t\"{}\": \"{}\",", stringify!($arg), ($arg).to_string());)*
			let _ = writeln!(stdin, "\t}},");
			$(let _ = writeln!(stdin, "\t\"data\": {}", serde_json::to_string($data).unwrap());)?
			let _ = writeln!(stdin, "}}");
		}

		let cmd = cmd.wait_with_output().unwrap();

		String::from_utf8_lossy(&cmd.stdout).to_string()
	}
}


/// modul obsahující endpointy pro CRUD na rezervaci
pub mod booking;

/// sbírá jednotlivé endpointy
///
/// přidání nového rgi
/// ```rust,no_run
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
