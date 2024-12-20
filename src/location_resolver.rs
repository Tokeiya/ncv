use super::location_error::LocationError;
use dirs;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const PLAT_FORM_PATH: &str = r#"tools\user\platform.json"#;

#[cfg(target_os = "windows")]
const NODE_DIR_PATH: &str = r#"tools\image\node\"#;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const PLAT_FORM_PATH: &str = r#"tools/user/platform.json"#;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const NODE_DIR_PATH: &str = r#"tools/image/node/"#;

fn check_exists(path: &OsStr) -> Result<bool, LocationError> {
	Ok(fs::exists(path)?)
}

fn windows_proc() -> Result<PathBuf, LocationError> {
	let mut dir = dirs::data_local_dir().ok_or(LocationError::FolderNotFound(
		r#"%LOCALAPPDATA%"#.to_string(),
	))?;

	dir.push(r#"Volta\"#);
	if !check_exists(dir.as_os_str())? {
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

pub fn get_folder() -> Result<PathBuf, LocationError> {
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

pub fn get_config_path() -> Result<PathBuf, LocationError> {
	let mut path = get_folder()?;
	path.push(PLAT_FORM_PATH);

	if !fs::exists(path.as_path())? {
		Err(LocationError::ConfigNotFound)
	} else {
		Ok(path)
	}
}

pub fn get_node_path() -> Result<PathBuf, LocationError> {
	let mut path = get_folder()?;
	path.push(NODE_DIR_PATH);

	if !fs::exists(path.as_path())? {
		let path = path.to_str().map_or("N/A", |x| x).to_string();
		Err(LocationError::FolderNotFound(path))
	} else {
		Ok(path)
	}
}
