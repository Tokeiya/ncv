use super::location_error::LocationError;
use dirs;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

//node cache
//C:\Users\net_s\AppData\Local\Volta\tools\inventory\node

//image
//C:\Users\net_s\AppData\Local\Volta\tools\image\node

#[cfg(target_os = "windows")]
mod paths {
	pub const PLATFORM: &str = r#"tools\user\platform.json"#;
	pub const NODE_IMAGE_DIR: &str = r#"tools\image\node\"#;
	pub const NODE_INVENTORY_DIR: &str = r#"tools\inventory\node\"#;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod paths {
	pub const PLATFORM: &str = r#"tools/user/platform.json"#;
	pub const NODE_IMAGE_DIR: &str = r#"tools/image/node/"#;
	pub const NODE_INVENTORY_DIR: &str = r#"tools/inventory/node/"#;
}

fn check_exists(path: PathBuf) -> Result<PathBuf, LocationError> {
	if !fs::exists(path.as_path())? {
		let path = path.to_str().map_or("N/A", |x| x).to_string();
		Err(LocationError::FolderNotFound(path))
	} else {
		Ok(path)
	}
}

fn windows_proc() -> Result<PathBuf, LocationError> {
	let mut dir = dirs::data_local_dir().ok_or(LocationError::FolderNotFound(
		r#"%LOCALAPPDATA%"#.to_string(),
	))?;

	dir.push(r#"Volta\"#);
	check_exists(dir)
}

fn linux_proc() -> Result<PathBuf, LocationError> {
	let mut dir = dirs::home_dir().ok_or(LocationError::FolderNotFound("~/".to_string()))?;
	dir.push(".volta/");

	if !fs::exists(dir.as_os_str())? {
		let path = if let Some(p) = dir.to_str() {
			p.to_string()
		} else {
			"N/A".to_string()
		};

		Err(LocationError::FolderNotFound(path))
	} else {
		Ok(dir)
	}
}

fn mac_proc() -> Result<PathBuf, LocationError> {
	linux_proc()
}

pub fn volta_folder() -> Result<PathBuf, LocationError> {
	if cfg!(target_os = "windows") {
		windows_proc()
	} else if cfg!(target_os = "linux") {
		linux_proc()
	} else if cfg!(target_os = "macos") {
		mac_proc()
	} else {
		Err(LocationError::UnexpectedOs)
	}
}

pub fn platform_path() -> Result<PathBuf, LocationError> {
	let mut path = volta_folder()?;
	path.push(paths::PLATFORM);

	if !fs::exists(path.as_path())? {
		Err(LocationError::ConfigNotFound)
	} else {
		Ok(path)
	}
}

pub fn node_inventory_path() -> Result<PathBuf, LocationError> {
	let mut path = volta_folder()?;
	path.push(paths::NODE_INVENTORY_DIR);

	check_exists(path)
}

pub fn node_image_path() -> Result<PathBuf, LocationError> {
	let mut path = volta_folder()?;
	path.push(paths::NODE_IMAGE_DIR);

	check_exists(path)
}
