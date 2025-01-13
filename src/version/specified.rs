use super::parse_error::ParseError;
use super::specified_element::SpecifiedElement;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct Specified {
	major: SpecifiedElement,
	minor: SpecifiedElement,
	patch: SpecifiedElement,
}

impl Display for Specified {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

impl Specified {
	pub fn new(major: SpecifiedElement, minor: SpecifiedElement, patch: SpecifiedElement) -> Self {
		Self {
			major,
			minor,
			patch,
		}
	}

	pub fn major(&self) -> u64 {
		self.major.value()
	}

	pub fn minor(&self) -> u64 {
		self.minor.value()
	}

	pub fn patch(&self) -> u64 {
		self.patch.value()
	}
}

impl FromStr for Specified {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		static REG: LazyLock<Regex> =
			LazyLock::new(|| Regex::new("^(\\d+)\\.(\\d+)\\.(\\d+)$").unwrap());

		if let Some(cap) = REG.captures(s) {
			let major = cap
				.get(1)
				.unwrap()
				.as_str()
				.parse::<u64>()
				.map_err(|_| ParseError::from(s))?;

			let minor = cap
				.get(2)
				.unwrap()
				.as_str()
				.parse::<u64>()
				.map_err(|_| ParseError::from(s))?;

			let patch = cap
				.get(3)
				.unwrap()
				.as_str()
				.parse::<u64>()
				.map_err(|_| ParseError::from(s))?;

			Ok(Specified::new(
				SpecifiedElement::from(major),
				SpecifiedElement::from(minor),
				SpecifiedElement::from(patch),
			))
		} else {
			Err(ParseError::from(s))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn fixture() -> Specified {
		Specified::new(
			SpecifiedElement::from(12),
			SpecifiedElement::from(345),
			SpecifiedElement::from(6789),
		)
	}

	#[test]
	fn new() {
		let actual = fixture();
		let expected = Specified {
			major: SpecifiedElement::from(12),
			minor: SpecifiedElement::from(345),
			patch: SpecifiedElement::from(6789),
		};

		assert_eq!(actual.major, expected.major);
		assert_eq!(actual.minor, expected.minor);
		assert_eq!(actual.patch, expected.patch);
	}

	#[test]
	fn major() {
		assert_eq!(fixture().major(), 12);
	}

	#[test]
	fn minor() {
		assert_eq!(fixture().minor(), 345);
	}

	#[test]
	fn patch() {
		assert_eq!(fixture().patch(), 6789);
	}

	#[test]
	fn from_str() {
		let actual = Specified::from_str("12.345.6789").unwrap();
		let expected = fixture();

		assert_eq!(actual.major, expected.major);
		assert_eq!(actual.minor, expected.minor);
		assert_eq!(actual.patch, expected.patch);
	}

	#[test]
	fn invalid_from_str() {
		assert!(Specified::from_str("12.345").is_err());
		assert!(Specified::from_str("12.345.6789.1011").is_err());
		assert!(Specified::from_str("12.345.6789.").is_err());
		assert!(Specified::from_str("12.*").is_err());
	}

	#[test]
	fn overflow_from_str() {
		assert!(Specified::from_str("18446744073709551616.345.6789").is_err());
		assert!(Specified::from_str("12.18446744073709551616.6789").is_err());
		assert!(Specified::from_str("12.345.18446744073709551616").is_err());
	}

	#[test]
	fn debug() {
		assert_eq!(
			format!("{:?}", fixture()),
			"Specified { major: SpecifiedElement(12), minor: SpecifiedElement(345), patch: SpecifiedElement(6789) }"
		);
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", fixture()), "12.345.6789");
	}
}
