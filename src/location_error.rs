use std::io;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum LocationError {
	#[error(transparent)]
	IoError(#[from] io::Error),
	#[error("Unexpected OS")]
	UnexpectedOs,
	#[error("Folder not found")]
	FolderNotFound(#[from] String),
	#[error("Data local dir not found")]
	DataLocalDirNotFound,
	#[error("Home dir not found")]
	HomeDirNotFound,
}
