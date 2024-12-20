use super::location_error::LocationError;
use dirs;
use std::path::PathBuf;

use std::fs;

pub fn get_folder() -> Result<PathBuf, LocationError> {
	if cfg!(target_os = "windows") {
		let mut path = dirs::data_local_dir().map_or_else();
		path.push(r#"\.volta\"#);

		let a = fs::metadata(&path).map_or(None, |x| Some(x))?;
	} else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
		todo!()
	} else {
		None
	}
}
