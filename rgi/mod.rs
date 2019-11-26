use rocket::Route;

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
			$(let _ = writeln!(stdin, "\t\"data\": {{ {} }}",
				{
					let mut s = serde_json::to_string($data).unwrap();
					s = s.chars().skip(1).collect();
					s.pop();
					s
				});
			)?
			let _ = writeln!(stdin, "}}");
		}

		let cmd = cmd.wait_with_output().unwrap();

		String::from_utf8_lossy(&cmd.stdout).to_string()
	}
}

mod booking;

pub fn routes() -> Vec<Route> {
	let mut routes = vec![];

	/*routes() funkce volat tady*/
	routes.extend(self::booking::routes());

	routes
}
