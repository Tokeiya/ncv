use super::parse_error::ParseError;
use super::strict_element::StrictElement;
use std::str::FromStr;

pub struct Specified {
	major: StrictElement,
	minor: StrictElement,
	patch: StrictElement,
}

impl Specified {
	pub fn new(major: StrictElement, minor: StrictElement, patch: StrictElement) -> Self {
		Self {
			major,
			minor,
			patch,
		}
	}

	pub fn major(&self) -> u64 {
		todo!()
	}

	pub fn minor(&self) -> u64 {
		todo!()
	}

	pub fn patch(&self) -> u64 {
		todo!()
	}
}

impl FromStr for Specified {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn fixture() -> Specified {
		Specified::new(StrictElement(12), StrictElement(345), StrictElement(6789))
	}
}
