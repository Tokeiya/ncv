use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("{} is unexpected target.",.0)]
	UnexpectedTarget(String),

	#[error("{} is unexpected version specification.",.0)]
	UnexpectedVersion(String),
}
