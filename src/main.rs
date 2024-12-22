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
use std::error::Error;
use std::fs;

fn err_to_string<T, E>(result: Result<T, E>) -> Result<T, String>
where
	E: Error,
{
	Ok(result.map_err(|e| e.to_string())?)
}

trait Foo {
	type T;
	fn err_to_string(self) -> Result<Self::T, String>;
}

impl<T, E: Error> Foo for Result<T, E> {
	type T = T;

	fn err_to_string(self) -> Result<Self::T, String> {
		Ok(self.map_err(|e| e.to_string())?)
	}
}

fn main() -> Result<(), String> {
	display_all()?;
	Ok(())
}

fn g() -> Result<(), String> {
	println!(
		"🧹Node cleaner for volta🧹 version {}",
		env!("CARGO_PKG_VERSION")
	);
	let arg = CommandArgs::parse();
	let version =
		node_version::NodeVersion::new(arg.node().to_string()).ok_or("Invalid node version")?;

	println!("n:{}", arg.node());

	let tmp = location_resolver::platform_path().map_err(|e| e.to_string())?;
	let conf = platform::Platform::from_read(fs::File::open(tmp).map_err(|e| e.to_string())?)
		.map_err(|e| e.to_string())?;

	if version.to_string() == conf.node_runtime() {
		return Err("Node version is activated".to_string());
	}

	remove_node(version.to_string())?;

	Ok(())
}

fn remove_node(version: String) -> Result<(), String> {
	todo!()
}

fn display_all() -> Result<(), String> {
	println!(
		"VoltaRoot:{:?}",
		location_resolver::volta_folder().err_to_string()?
	);
	println!(
		"PlatformJson:{:?}",
		location_resolver::platform_path().err_to_string()?
	);
	println!(
		"NodeImagePath:{:?}",
		location_resolver::node_image_path().err_to_string()?
	);
	println!(
		"NodeInventoryPath:{:?}",
		location_resolver::node_inventory_path().err_to_string()?
	);

	let file = fs::File::open(location_resolver::platform_path().unwrap()).unwrap();
	let conf = platform::Platform::from_read(file).err_to_string()?;

	println!("node_runtime:{}", conf.node_runtime());
	println!("node_npm:{:?}", conf.node_npm());
	println!("pnpm:{:?}", conf.pnpm());
	println!("yarn:{:?}", conf.yarn());

	Ok(())
}
