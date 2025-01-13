use crate::version::element::Element;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct SpecifiedElement(u64);

impl From<u64> for SpecifiedElement {
	fn from(value: u64) -> Self {
		SpecifiedElement(value)
	}
}

impl TryFrom<&str> for SpecifiedElement {
	type Error = std::num::ParseIntError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(SpecifiedElement(value.parse()?))
	}
}

impl Display for SpecifiedElement {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Ok(write!(f, "{}", self.0)?)
	}
}

impl SpecifiedElement {
	pub fn value(&self) -> u64 {
		self.0
	}
}

impl PartialEq<Element> for SpecifiedElement {
	fn eq(&self, other: &Element) -> bool {
		match other {
			Element::Specified(other) => self == other,
			Element::Any => true,
		}
	}
}

impl PartialOrd<Element> for SpecifiedElement {
	fn partial_cmp(&self, other: &Element) -> Option<Ordering> {
		match other {
			Element::Specified(other) => self.partial_cmp(other),
			Element::Any => None,
		}
	}

	fn ge(&self, other: &Element) -> bool {
		match other {
			Element::Specified(elem) => self.ge(elem),
			Element::Any => true,
		}
	}

	fn gt(&self, other: &Element) -> bool {
		match other {
			Element::Specified(elem) => self.gt(elem),
			Element::Any => true,
		}
	}

	fn le(&self, other: &Element) -> bool {
		match other {
			Element::Specified(elem) => self.le(elem),
			Element::Any => true,
		}
	}

	fn lt(&self, other: &Element) -> bool {
		match other {
			Element::Specified(elem) => self.lt(elem),
			Element::Any => true,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use eq_ord_tester::ord::simple_ord_test;
	use eq_ord_tester::simple_partial_eq::*;

	#[test]
	fn element_partial_eq() {
		let fixture = SpecifiedElement(42);
		let tgt = Element::Specified(SpecifiedElement(42));

		assert_eq!(fixture, tgt);
		assert_eq!(tgt, fixture);

		let any = Element::Any;

		assert_eq!(any, fixture);
		assert_eq!(fixture, any);

		let ne = Element::Specified(SpecifiedElement(43));
		assert_ne!(ne, fixture);
		assert_ne!(fixture, ne);
	}

	#[test]
	fn element_partial_ord() {
		let fixture = SpecifiedElement(42);
		let less = Element::from(41);
		let greater = Element::from(43);
		let equal = Element::from(42);

		assert!(matches!(fixture.partial_cmp(&less), Some(act) if act == Ordering::Greater));
		assert!(matches!(fixture.partial_cmp(&greater), Some(act) if act == Ordering::Less));
		assert!(matches!(fixture.partial_cmp(&equal), Some(act) if act == Ordering::Equal));

		assert!(fixture.partial_cmp(&Element::Any).is_none());
	}

	#[test]
	fn ge() {
		let fixture = SpecifiedElement(42);
		let less = Element::from(41);
		let greater = Element::from(43);
		let equal = Element::from(42);
		let any = Element::Any;

		assert!(fixture.ge(&less));
		assert!(fixture.ge(&equal));
		assert!(!fixture.ge(&greater));
		assert!(fixture.ge(&any));
	}

	#[test]
	fn gt() {
		let fixture = SpecifiedElement(42);
		let less = Element::from(41);
		let greater = Element::from(43);
		let equal = Element::from(42);
		let any = Element::Any;

		assert!(fixture.gt(&less));
		assert!(!fixture.gt(&equal));
		assert!(!fixture.gt(&greater));
		assert!(fixture.gt(&any));
	}

	#[test]
	fn le() {
		let fixture = SpecifiedElement(42);
		let less = Element::from(41);
		let greater = Element::from(43);
		let equal = Element::from(42);
		let any = Element::Any;

		assert!(!fixture.le(&less));
		assert!(fixture.le(&equal));
		assert!(fixture.le(&greater));
		assert!(fixture.le(&any));
	}

	#[test]
	fn lt() {
		let fixture = SpecifiedElement(42);
		let less = Element::from(41);
		let greater = Element::from(43);
		let equal = Element::from(42);
		let any = Element::Any;

		assert!(!fixture.lt(&less));
		assert!(!fixture.lt(&equal));
		assert!(fixture.lt(&greater));
		assert!(fixture.lt(&any));
	}

	#[test]
	fn try_from_str() {
		let fixture = SpecifiedElement::try_from("42").unwrap();
		assert_eq!(fixture.0, 42);

		assert!(SpecifiedElement::try_from("4 3").is_err())
	}

	#[test]
	fn from_u64() {
		let fixture = SpecifiedElement::from(42);
	}
	#[test]
	fn eq() {
		symmetric_test(SpecifiedElement(42), SpecifiedElement(42));
		reflexive_test(SpecifiedElement(42));
		transitive_test(
			SpecifiedElement(42),
			SpecifiedElement(42),
			SpecifiedElement(42),
		);
		not_eq_test(SpecifiedElement(42), SpecifiedElement(43));
	}

	#[test]
	fn ord() {
		simple_ord_test(
			SpecifiedElement(42),
			SpecifiedElement(43),
			SpecifiedElement(41),
			SpecifiedElement(42),
		)
	}

	#[test]
	fn debug() {
		let fixture = SpecifiedElement(42);
		assert_eq!(format!("{fixture:?}"), "SpecifiedElement(42)");
	}

	#[test]
	fn display() {
		let fixture = SpecifiedElement(42);
		assert_eq!(format!("{fixture}"), "42");
	}

	#[test]
	fn value() {
		let fixture = SpecifiedElement(42);
		assert_eq!(fixture.value(), 42);
	}
}
