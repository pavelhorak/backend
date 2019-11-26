use std::process::Command;
use std::env::set_current_dir;

fn main() {
	set_current_dir("frontend").expect("frontend missing");
	Command::new("make").spawn().expect("failed to build frontend");
}
