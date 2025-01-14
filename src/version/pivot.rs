use super::element::Element;
use super::parse_error::ParseError;
use super::specified_element::SpecifiedElement;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct Pivot {
	major: SpecifiedElement,
	minor: Element,
	patch: Element,
}

impl Pivot {
	pub fn new(major: SpecifiedElement, minor: Element, patch: Element) -> Self {
		Self {
			major,
			minor,
			patch,
		}
	}

	pub fn major(&self) -> &SpecifiedElement {
		&self.major
	}

	pub fn minor(&self) -> &Element {
		&self.minor
	}

	pub fn patch(&self) -> &Element {
		&self.patch
	}
}

impl Display for Pivot {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut ret = String::new();

		ret.push_str(&self.major.to_string());
		ret.push('.');

		if let Element::Specified(element) = &self.minor {
			ret.push_str(&element.to_string());
		} else {
			ret.push('*');
		}

		ret.push('.');

		if let Element::Specified(element) = &self.patch {
			ret.push_str(&element.to_string());
		} else {
			ret.push('*');
		}

		write!(f, "{}", ret)
	}
}
impl FromStr for Pivot {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		static MAJOR: LazyLock<regex::Regex> =
			LazyLock::new(|| regex::Regex::new(r"^(\d+)(\.\*)?$").unwrap());

		static MINOR: LazyLock<regex::Regex> =
			LazyLock::new(|| regex::Regex::new(r"^(\d+)\.(\d+)(\.\*)?$").unwrap());

		static PATCH: LazyLock<regex::Regex> =
			LazyLock::new(|| regex::Regex::new(r"^(\d+)\.(\d+)\.(\d+)$").unwrap());

		if let Some(cap) = MAJOR.captures(s) {
			let major = cap
				.get(1)
				.unwrap()
				.as_str()
				.parse::<u64>()
				.map_err(|_| ParseError::from(s))?;

			Ok(Pivot::new(
				SpecifiedElement::from(major),
				Element::Any,
				Element::Any,
			))
		} else if let Some(cap) = MINOR.captures(s) {
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

			Ok(Pivot::new(
				SpecifiedElement::from(major),
				Element::from(minor),
				Element::Any,
			))
		} else if let Some(cap) = PATCH.captures(s) {
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
			Ok(Pivot::new(
				SpecifiedElement::from(major),
				Element::from(minor),
				Element::from(patch),
			))
		} else {
			Err(ParseError::from(s))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn fixture() -> Pivot {
		Pivot::new(
			SpecifiedElement::from(42),
			Element::from(43),
			Element::from(44),
		)
	}

	fn any_fixture() -> Pivot {
		Pivot::new(SpecifiedElement::from(42), Element::Any, Element::Any)
	}
	#[test]
	fn new() {
		let fixture = Pivot::new(
			SpecifiedElement::from(42),
			Element::from(43),
			Element::from(44),
		);

		assert_eq!(&fixture.major, &SpecifiedElement::from(42));
		assert!(matches!(&fixture.minor,Element::Specified(x) if x.value()==43));
		assert!(matches!(&fixture.patch,Element::Specified(x) if x.value()==44));
	}

	#[test]
	fn major() {
		let fixture = fixture();
		assert_eq!(fixture.major().value(), 42);
	}

	#[test]
	fn minor() {
		let fixture = fixture();
		assert!(matches!(fixture.minor(),Element::Specified(act) if act.value()==43));

		let fixture = any_fixture();
		assert!(matches!(fixture.minor(), Element::Any));
	}

	#[test]
	fn patch() {
		let fixture = fixture();
		assert!(matches!(fixture.patch(),Element::Specified(act) if act.value()==44));

		let fixture = any_fixture();
		assert!(matches!(fixture.patch(), Element::Any));
	}

	#[test]
	fn full_spec_from_str() {
		let fixture = Pivot::from_str("42.43.44").unwrap();
		assert_eq!(fixture.major().value(), 42);
		assert!(matches!(fixture.minor(),Element::Specified(act) if act.value()==43));
		assert!(matches!(fixture.patch(),Element::Specified(act) if act.value()==44));
	}

	#[test]
	fn minor_any_from_str() {
		let fixture = Pivot::from_str("42.*").unwrap();
		assert_eq!(fixture.major().value(), 42);
		assert!(matches!(fixture.minor(), Element::Any));
		assert!(matches!(fixture.patch(), Element::Any));

		let fixture = Pivot::from_str("42").unwrap();
		assert_eq!(fixture.major().value(), 42);
		assert!(matches!(fixture.minor(), Element::Any));
		assert!(matches!(fixture.patch(), Element::Any));
	}

	#[test]
	fn patch_any_from_str() {
		let fixture = Pivot::from_str("42.43.*").unwrap();
		assert_eq!(fixture.major().value(), 42);
		assert!(matches!(fixture.minor(),Element::Specified(act) if act.value()==43));
		assert!(matches!(fixture.patch(), Element::Any));

		let fixture = Pivot::from_str("42.43").unwrap();
		assert_eq!(fixture.major().value(), 42);
		assert!(matches!(fixture.minor(),Element::Specified(act) if act.value()==43));
	}

	#[test]
	fn invalid_from_str() {
		assert!(Pivot::from_str("*").is_err());
		assert!(Pivot::from_str("42.*.43").is_err());
	}

	#[test]
	fn ovf_from_str() {
		assert!(Pivot::from_str("18446744073709551616.43.44").is_err());
		assert!(Pivot::from_str("42.18446744073709551616.44").is_err());
		assert!(Pivot::from_str("42.43.18446744073709551616").is_err());
	}

	#[test]
	fn debug() {
		let fixture = fixture();
		println!("{:?}", fixture);

		assert_eq!(
			format!("{:?}", fixture),
			"Pivot { major: SpecifiedElement(42), minor: Specified(SpecifiedElement(43)), patch: Specified(SpecifiedElement(44)) }"
		);

		assert_eq!(
			format!("{:?}", any_fixture()),
			"Pivot { major: SpecifiedElement(42), minor: Any, patch: Any }"
		);
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", fixture()), "42.43.44");
		assert_eq!(format!("{}", any_fixture()), "42.*.*");
	}
}
