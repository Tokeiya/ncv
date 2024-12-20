mod location_error;
mod location_resolver;
mod platform;
mod platform_load_error;

fn main() {
	let a = location_resolver::get_folder();
	println!("{a:?}");

	println!("{:?}", location_resolver::get_config_path());
	println!("{:?}", location_resolver::get_node_path());
}
