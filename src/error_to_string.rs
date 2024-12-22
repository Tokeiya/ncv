use std::error::Error;

pub trait ErrorToString {
	type T;
	fn conv_to_str(self) -> Result<Self::T, String>;
}

impl<T, E: Error> ErrorToString for Result<T, E> {
	type T = T;

	fn conv_to_str(self) -> Result<Self::T, String> {
		Ok(self.map_err(|e| e.to_string())?)
	}
}
