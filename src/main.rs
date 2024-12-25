mod command;
mod error_to_string;
mod location_error;
mod location_resolver;
mod platform;
mod platform_load_error;

use crate::error_to_string::ErrorToString;
use clap::Parser;
use regex;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
fn main() {
	let a = "110";
	let b = "20";

	let b = a.parse::<u64>();
}

fn remove_node(version: String) -> Result<(), String> {
	todo!()
}

fn display_all() -> Result<(), String> {
	println!(
		"VoltaRoot:{:?}",
		location_resolver::volta_folder().conv_to_str()?
	);
	println!(
		"PlatformJson:{:?}",
		location_resolver::platform_path().conv_to_str()?
	);
	println!(
		"NodeImagePath:{:?}",
		location_resolver::node_image_path().conv_to_str()?
	);
	println!(
		"NodeInventoryPath:{:?}",
		location_resolver::node_inventory_path().conv_to_str()?
	);

	let file = fs::File::open(location_resolver::platform_path().unwrap()).unwrap();
	let conf = platform::Platform::from_read(file).conv_to_str()?;

	println!("node_runtime:{}", conf.node_runtime());
	println!("node_npm:{:?}", conf.node_npm());
	println!("pnpm:{:?}", conf.pnpm());
	println!("yarn:{:?}", conf.yarn());

	Ok(())
}
