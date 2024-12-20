mod location_error;
mod location_resolver;
mod platform;
mod platform_load_error;

use dirs;

fn main() {
	let local = dirs::data_local_dir();
	println!("{:?}", &local);

	let home = dirs::home_dir();
	println!("{:?}", home);
}
