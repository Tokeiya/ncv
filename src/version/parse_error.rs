use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Failed to parse version: {0}")]
pub struct ParseError(String);
