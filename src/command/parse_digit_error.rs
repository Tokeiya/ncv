use std::num::ParseIntError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("{} is unexpected input.",.0)]
	ParseError(String),
	#[error(transparent)]
	ConvertError(#[from] ParseIntError),
}
