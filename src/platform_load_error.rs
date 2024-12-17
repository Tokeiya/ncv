use thiserror::Error as ThisError;
use serde_json::Error as SerdeJsonError;

#[derive(ThisError,Debug)]
pub enum Error{
    #[error(transparent)]
    Serde(#[from] SerdeJsonError),
    #[error("Entry {0} not found")]
    EntryNotFound(String),
    #[error("Entry {0} is unexpected element types.")]
    UnexpectedValueType(String)
}