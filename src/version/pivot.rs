use super::element::Element;
use super::parse_error::ParseError;
use super::specified_element::SpecifiedElement;
use crate::version::specified::Specified;
use std::cmp::Ordering;
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

impl PartialEq<Specified> for Pivot {
	fn eq(&self, other: &Specified) -> bool {
		self.major() == other.major()
			&& self.minor() == other.minor()
			&& self.patch() == other.patch()
	}
}

impl PartialOrd<Specified> for Pivot {
	fn partial_cmp(&self, other: &Specified) -> Option<Ordering> {
		let m = self.major().partial_cmp(other.major()).unwrap();
		let n = self.minor().partial_cmp(other.minor());
		let p = self.patch().partial_cmp(other.patch());

		match m {
			Ordering::Less => Some(Ordering::Less),
			Ordering::Equal => {
				if let (Some(n), Some(p)) = (n, p) {
					match n {
						Ordering::Less => Some(Ordering::Less),
						Ordering::Equal => Some(p),
						Ordering::Greater => Some(Ordering::Greater),
					}
				} else {
					None
				}
			}
			Ordering::Greater => Some(Ordering::Greater),
		}
	}

	fn ge(&self, other: &Specified) -> bool {
		if let Some(ord) = self.partial_cmp(other) {
			match ord {
				Ordering::Less => false,
				Ordering::Equal => true,
				Ordering::Greater => true,
			}
		} else {
			true
		}
	}

	fn gt(&self, other: &Specified) -> bool {
		todo!()
	}

	fn le(&self, other: &Specified) -> bool {
		todo!()
	}

	fn lt(&self, other: &Specified) -> bool {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::super::specified::Specified;
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
	fn partial_cmp() {
		let major = Pivot::from_str("42").unwrap();
		let minor = Pivot::from_str("42.43").unwrap();
		let patch = fixture();

		//none
		let bind = Specified::from_str("42.43.44").unwrap();
		assert!(major.partial_cmp(&bind).is_none());
		assert!(minor.partial_cmp(&bind).is_none());

		//equal
		assert_eq!(patch.partial_cmp(&bind), Some(Ordering::Equal));

		//greater
		let less = Specified::from_str("42.43.43").unwrap();
		assert_eq!(patch.partial_cmp(&less), Some(Ordering::Greater));

		//less
		let greater = Specified::from_str("42.43.45").unwrap();
		assert_eq!(patch.partial_cmp(&greater), Some(Ordering::Less));
	}

	#[test]
	fn foo() {
		let minor = Pivot::from_str("42.43").unwrap();

		dbg!(minor.ge(&Specified::from_str("42.44.90").unwrap()));
		dbg!(minor.partial_cmp(&Specified::from_str("42.44.90").unwrap()));
		assert!(!minor.ge(&Specified::from_str("42.44.90").unwrap()));
	}

	#[test]
	fn ge() {
		//42.43.44
		let major = Pivot::from_str("42").unwrap();
		let minor = Pivot::from_str("42.43").unwrap();
		let patch = fixture();

		//equal
		assert!(patch.ge(&Specified::from_str("42.43.44").unwrap()));

		//greater
		assert!(patch.ge(&Specified::from_str("42.43.41").unwrap()));
		assert!(patch.ge(&Specified::from_str("42.42.45").unwrap()));
		assert!(patch.ge(&Specified::from_str("41.50.60").unwrap()));

		assert!(minor.ge(&Specified::from_str("42.43.90").unwrap()));
		assert!(minor.ge(&Specified::from_str("42.4.99").unwrap()));

		assert!(major.ge(&Specified::from_str("42.43.43").unwrap()));
		assert!(major.ge(&Specified::from_str("42.44.90").unwrap()));

		// //less
		assert!(!patch.ge(&Specified::from_str("42.43.45").unwrap()));
		assert!(!patch.ge(&Specified::from_str("42.44.44").unwrap()));
		assert!(!patch.ge(&Specified::from_str("43.43.44").unwrap()));

		assert!(!minor.ge(&Specified::from_str("42.44.90").unwrap()));
		assert!(!minor.ge(&Specified::from_str("43.4.99").unwrap()));

		assert!(!major.ge(&Specified::from_str("43.900.900").unwrap()));
	}

	#[test]
	fn gt() {
		let major = Pivot::from_str("42").unwrap();
		let minor = Pivot::from_str("42.43").unwrap();
		let patch = fixture();

		//great
		assert!(patch.gt(&Specified::from_str("42.43.43").unwrap()));
		assert!(patch.gt(&Specified::from_str("42.42.50").unwrap()));
		assert!(patch.gt(&Specified::from_str("41.43.44").unwrap()));

		assert!(minor.gt(&Specified::from_str("42.42.99").unwrap()));
		assert!(minor.gt(&Specified::from_str("41.45.46").unwrap()));

		assert!(major.gt(&Specified::from_str("41.50.44").unwrap()));

		//less
		assert!(!patch.gt(&Specified::from_str("42.43.44").unwrap()));
		assert!(!patch.gt(&Specified::from_str("42.43.45").unwrap()));
		assert!(!patch.gt(&Specified::from_str("42.44.44").unwrap()));

		assert!(!minor.gt(&Specified::from_str("42.43.44").unwrap()));
		assert!(!minor.gt(&Specified::from_str("42.43.45").unwrap()));

		assert!(!major.gt(&Specified::from_str("42.45.46").unwrap()));
	}

	#[test]
	fn le() {
		let major = Pivot::from_str("42").unwrap();
		let minor = Pivot::from_str("42.43").unwrap();
		let patch = fixture();

		//equal
		assert!(patch.le(&Specified::from_str("42.43.44").unwrap()));

		//less
		assert!(patch.le(&Specified::from_str("42.43.45").unwrap()));
		assert!(patch.le(&Specified::from_str("42.44.1").unwrap()));
		assert!(patch.le(&Specified::from_str("43.0.44").unwrap()));

		assert!(minor.le(&Specified::from_str("42.44.0").unwrap()));
		assert!(minor.le(&Specified::from_str("43.0.0").unwrap()));

		assert!(major.le(&Specified::from_str("43.0.0").unwrap()));
	}

	#[test]
	fn lt() {
		let major = Pivot::from_str("42").unwrap();
		let minor = Pivot::from_str("42.43").unwrap();
		let patch = fixture();

		//less
		assert!(patch.lt(&Specified::from_str("42.43.45").unwrap()));
		assert!(patch.lt(&Specified::from_str("42.44.0").unwrap()));
		assert!(patch.lt(&Specified::from_str("43.0.0").unwrap()));

		assert!(minor.lt(&Specified::from_str("42.44.0").unwrap()));
		assert!(minor.lt(&Specified::from_str("43.0.0").unwrap()));

		assert!(major.lt(&Specified::from_str("43.0.0").unwrap()));
	}

	#[test]
	fn specified_partial_eq() {
		let specified = Specified::from_str("42.43.44").unwrap();
		assert_eq!(fixture(), specified);

		assert_ne!(fixture(), Specified::from_str("42.43.45").unwrap());
		assert_ne!(fixture(), Specified::from_str("42.44.43").unwrap());

		assert_ne!(fixture(), Specified::from_str("43.44.44").unwrap());
		assert_ne!(fixture(), Specified::from_str("43.45.44").unwrap());

		assert_ne!(fixture(), Specified::from_str("43.43.44").unwrap());
		assert_ne!(fixture(), Specified::from_str("41.43.44").unwrap());
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
