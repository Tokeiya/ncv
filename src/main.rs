mod command_args;
mod location_error;
mod location_resolver;
mod node_version;
mod platform;
mod platform_load_error;
mod validator;

use clap::Parser;
use command_args::CommandArgs;
use regex;
use std::fs;
fn main() {
	let reg = regex::Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<revision>\d+)$").unwrap();

	let a = reg.captures("1234.334.42").unwrap();

	let b = &a["major"];

	println!(
		"major:{} minor:{} rev:{}",
		&a["major"], &a["minor"], &a["revision"]
	)
}

fn g() {
	println!(
		"🧹Node cleaner for volta🧹 version {}",
		env!("CARGO_PKG_VERSION")
	);
	let arg = CommandArgs::parse();

	println!("n:{}", arg.node())
}

fn f() {
	let a = location_resolver::get_folder();
	println!("{a:?}");

	println!("{:?}", location_resolver::get_config_path());
	println!("{:?}", location_resolver::get_node_path());

	let file = fs::File::open(location_resolver::get_config_path().unwrap()).unwrap();
	let conf = platform::Platform::from_read(file);

	if let Err(e) = conf {
		println!("{e:?}")
	}
}
