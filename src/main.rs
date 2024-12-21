mod location_error;
mod location_resolver;
mod platform;
mod platform_load_error;

use std::fs;

fn main() {
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
