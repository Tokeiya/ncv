use std::io;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum LocationError {
	#[error(transparent)]
	IoError(#[from] io::Error),
	#[error("Unexpected OS")]
	UnexpectedOs,
	#[error("Folder {} not found",.0)]
	FolderNotFound(String),
	#[error("platform.json not found")]
	ConfigNotFound,
}
