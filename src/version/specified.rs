use super::parse_error::ParseError;
use super::specified_element::SpecifiedElement;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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

	enum Targets {
		Major,
		Minor,
		Patch,
	}

	fn pivot() -> Specified {
		Specified::new(
			SpecifiedElement::from(12),
			SpecifiedElement::from(345),
			SpecifiedElement::from(6789),
		)
	}

	fn greater(target: Targets) -> Specified {
		match target {
			Targets::Major => Specified::new(
				SpecifiedElement::from(13),
				SpecifiedElement::from(345),
				SpecifiedElement::from(6789),
			),
			Targets::Minor => Specified::new(
				SpecifiedElement::from(12),
				SpecifiedElement::from(346),
				SpecifiedElement::from(6789),
			),
			Targets::Patch => Specified::new(
				SpecifiedElement::from(12),
				SpecifiedElement::from(345),
				SpecifiedElement::from(6790),
			),
		}
	}

	fn less(target: Targets) -> Specified {
		match target {
			Targets::Major => Specified::new(
				SpecifiedElement::from(11),
				SpecifiedElement::from(345),
				SpecifiedElement::from(6789),
			),
			Targets::Minor => Specified::new(
				SpecifiedElement::from(12),
				SpecifiedElement::from(344),
				SpecifiedElement::from(6789),
			),
			Targets::Patch => Specified::new(
				SpecifiedElement::from(12),
				SpecifiedElement::from(345),
				SpecifiedElement::from(6788),
			),
		}
	}

	#[test]
	fn eq() {
		assert_eq!(pivot(), pivot());
		assert_eq!(less(Targets::Major), less(Targets::Major));
		assert_eq!(greater(Targets::Minor), greater(Targets::Minor));

		assert_ne!(pivot(), greater(Targets::Major));
		assert_ne!(pivot(), greater(Targets::Minor));
		assert_ne!(pivot(), greater(Targets::Patch));

		assert_ne!(pivot(), less(Targets::Major));
		assert_ne!(pivot(), less(Targets::Minor));
		assert_ne!(pivot(), less(Targets::Patch));
	}

	#[test]
	fn partial_ord() {
		assert_eq!(
			pivot().partial_cmp(&pivot()),
			Some(std::cmp::Ordering::Equal)
		);

		assert_eq!(
			less(Targets::Major).partial_cmp(&pivot()),
			Some(std::cmp::Ordering::Less)
		);
		assert_eq!(
			less(Targets::Minor).partial_cmp(&pivot()),
			Some(std::cmp::Ordering::Less)
		);
		assert_eq!(
			less(Targets::Patch).partial_cmp(&pivot()),
			Some(std::cmp::Ordering::Less)
		);

		assert_eq!(
			greater(Targets::Major).partial_cmp(&pivot()),
			Some(std::cmp::Ordering::Greater)
		);
		assert_eq!(
			greater(Targets::Minor).partial_cmp(&pivot()),
			Some(std::cmp::Ordering::Greater)
		);
		assert_eq!(
			greater(Targets::Patch).partial_cmp(&pivot()),
			Some(std::cmp::Ordering::Greater)
		);
	}

	#[test]
	fn ord() {
		assert_eq!(pivot().cmp(&pivot()), std::cmp::Ordering::Equal);

		assert_eq!(less(Targets::Major).cmp(&pivot()), std::cmp::Ordering::Less);
		assert_eq!(less(Targets::Minor).cmp(&pivot()), std::cmp::Ordering::Less);
		assert_eq!(less(Targets::Patch).cmp(&pivot()), std::cmp::Ordering::Less);

		assert_eq!(
			greater(Targets::Major).cmp(&pivot()),
			std::cmp::Ordering::Greater
		);
		assert_eq!(
			greater(Targets::Minor).cmp(&pivot()),
			std::cmp::Ordering::Greater
		);
		assert_eq!(
			greater(Targets::Patch).cmp(&pivot()),
			std::cmp::Ordering::Greater
		);
	}

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
