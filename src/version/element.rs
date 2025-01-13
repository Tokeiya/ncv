use super::specified_element::SpecifiedElement;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Element {
	Specified(SpecifiedElement),
	Any,
}

impl From<u64> for Element {
	fn from(value: u64) -> Self {
		Element::Specified(SpecifiedElement::from(value))
	}
}

impl TryFrom<&str> for Element {
	type Error = std::num::ParseIntError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(Element::Specified(SpecifiedElement::try_from(value)?))
	}
}

impl PartialEq<SpecifiedElement> for Element {
	fn eq(&self, other: &SpecifiedElement) -> bool {
		match self {
			Element::Specified(elem) => elem == other,
			Element::Any => true,
		}
	}
}

impl Display for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Element::Specified(elem) => write!(f, "{}", elem),
			Element::Any => write!(f, "*"),
		}
	}
}

impl Element {
	fn unwrap_specified(&self) -> u64 {
		if let Element::Specified(elem) = self {
			elem.value()
		} else {
			panic!("Called unwrap_specified on Element::Any")
		}
	}

	fn is_any(&self) -> bool {
		matches!(self, Element::Any)
	}
}

impl PartialOrd<SpecifiedElement> for Element {
	fn partial_cmp(&self, other: &SpecifiedElement) -> Option<Ordering> {
		match self {
			Element::Specified(x) => x.partial_cmp(other),
			Element::Any => None,
		}
	}

	fn ge(&self, other: &SpecifiedElement) -> bool {
		match self {
			Element::Specified(x) => x.ge(other),
			Element::Any => true,
		}
	}

	fn gt(&self, other: &SpecifiedElement) -> bool {
		match self {
			Element::Specified(x) => x.gt(other),
			Element::Any => true,
		}
	}

	fn le(&self, other: &SpecifiedElement) -> bool {
		match self {
			Element::Specified(x) => x.le(other),
			Element::Any => true,
		}
	}

	fn lt(&self, other: &SpecifiedElement) -> bool {
		match self {
			Element::Specified(x) => x.lt(other),
			Element::Any => true,
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::specified_element::SpecifiedElement;
	use super::*;
	#[test]
	fn from_u64() {
		let fixture = Element::from(42);

		if let Element::Specified(elem) = fixture {
			assert_eq!(elem.value(), 42)
		}
	}

	#[test]
	fn try_from_str() {
		let fixture = Element::try_from("42").unwrap();
		assert_eq!(fixture.unwrap_specified(), 42);

		assert!(Element::try_from("4 3").is_err());
	}

	#[test]
	fn debug() {
		let fixture = Element::from(42);
		assert_eq!(format!("{:?}", fixture), "Specified(SpecifiedElement(42))");
		assert_eq!(format!("{:?}", Element::Any), "Any");
	}

	#[test]
	fn display() {
		let fixture = Element::from(42);
		assert_eq!(format!("{}", fixture), "42");
		assert_eq!(format!("{}", Element::Any), "*");
	}

	#[test]
	fn unwrap() {
		let fixture = Element::from(42);
		assert_eq!(fixture.unwrap_specified(), 42);
	}

	#[test]
	#[should_panic]
	fn invalid_unwrap() {
		let fixture = Element::Any;
		assert_eq!(fixture.unwrap_specified(), 42);
	}

	#[test]
	fn is_any() {
		let fixture = Element::from(42);
		assert!(!fixture.is_any());

		let fixture = Element::Any;
		assert!(fixture.is_any());
	}

	#[test]
	fn partial_eq() {
		let fixture = Element::from(42);
		let eq = SpecifiedElement::from(42);

		assert_eq!(fixture, eq);
		assert_eq!(eq, fixture);

		let fixture = Element::Any;

		assert_eq!(fixture, eq);
		assert_eq!(eq, fixture);

		let fixture = Element::from(43);
		assert_ne!(fixture, eq);
	}

	#[test]
	fn partial_ord() {
		let fixture = Element::from(42);
		let less = SpecifiedElement::from(41);
		let greater = SpecifiedElement::from(43);
		let equal = SpecifiedElement::from(42);

		assert!(matches!(fixture.partial_cmp(&less), Some(act) if act == Ordering::Greater));
		assert!(matches!(fixture.partial_cmp(&greater), Some(act) if act == Ordering::Less));
		assert!(matches!(fixture.partial_cmp(&equal), Some(act) if act == Ordering::Equal));

		let fixture = Element::Any;
		assert!(fixture.partial_cmp(&equal).is_none());
	}

	#[test]
	fn ge() {
		let fixture = Element::from(42);
		let less = SpecifiedElement::from(41);
		let greater = SpecifiedElement::from(43);
		let equal = SpecifiedElement::from(42);

		assert!(fixture.ge(&less));
		assert!(fixture.ge(&equal));
		assert!(!fixture.ge(&greater));

		let fixture = Element::Any;
		assert!(fixture.ge(&equal));
	}

	#[test]
	fn gt() {
		let fixture = Element::from(42);
		let less = SpecifiedElement::from(41);
		let greater = SpecifiedElement::from(43);
		let equal = SpecifiedElement::from(42);

		assert!(fixture.gt(&less));
		assert!(!fixture.gt(&equal));
		assert!(!fixture.gt(&greater));

		let fixture = Element::Any;
		assert!(fixture.gt(&equal));
	}

	#[test]
	fn le() {
		let fixture = Element::from(42);
		let less = SpecifiedElement::from(41);
		let greater = SpecifiedElement::from(43);
		let equal = SpecifiedElement::from(42);

		assert!(fixture.le(&greater));
		assert!(fixture.le(&equal));
		assert!(!fixture.le(&less));

		let fixture = Element::Any;
		assert!(fixture.le(&equal));
	}

	#[test]
	fn lt() {
		let fixture = Element::from(42);
		let less = SpecifiedElement::from(41);
		let greater = SpecifiedElement::from(43);
		let equal = SpecifiedElement::from(42);

		assert!(fixture.lt(&greater));
		assert!(!fixture.lt(&equal));
		assert!(!fixture.lt(&less));

		let fixture = Element::Any;
		assert!(fixture.lt(&equal));
	}
}
