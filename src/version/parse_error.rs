use thiserror::Error;

#[derive(Debug, Error)]
#[error("Failed to parse version: {0}")]
pub struct ParseError(String);

impl From<&str> for ParseError {
	fn from(value: &str) -> Self {
		ParseError(value.to_string())
	}
}
