use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

pub const ANY_SIGN: char = '*';

#[derive(Debug)]
pub enum Element {
	Any,
	Specified(u64),
}

impl Display for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Element::Any => write!(f, "*"),
			Element::Specified(i) => write!(f, "{i}"),
		}
	}
}

impl PartialEq for Element {
	fn eq(&self, other: &Self) -> bool {
		if self.is_any() || other.is_any() {
			false
		} else {
			self.unwrap() == other.unwrap()
		}
	}
}

impl PartialOrd for Element {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.is_any() || other.is_any() {
			None
		} else {
			self.unwrap().partial_cmp(&other.unwrap())
		}
	}
}

impl Element {
	pub fn any() -> Self {
		Element::Any
	}

	pub fn is_any(&self) -> bool {
		match self {
			Element::Any => true,
			Element::Specified(_) => false,
		}
	}

	pub fn unwrap(&self) -> u64 {
		match self {
			Element::Any => panic!("Any"),
			Element::Specified(i) => *i,
		}
	}
}

impl From<u64> for Element {
	fn from(value: u64) -> Self {
		Self::Specified(value)
	}
}

impl TryFrom<&str> for Element {
	type Error = ParseIntError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let v = value.parse()?;
		Ok(Self::Specified(v))
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl Element {
		pub fn is(&self, expected: u64) {
			assert!(matches!(self,Element::Specified(i) if i==&expected))
		}
	}

	#[test]
	fn is_any_test() {
		let fixture = Element::Any;
		fixture.is_any();
	}

	#[test]
	fn invalid_is_any() {
		let fixture = Element::Specified(42);
		assert!(!fixture.is_any());
	}

	#[test]
	fn is_test() {
		for expected in 0..10 {
			let fixture = Element::Specified(expected);
			fixture.is(expected)
		}
	}

	#[test]
	#[should_panic]
	fn invalid_is_test() {
		let fixture = Element::Specified(42);
		fixture.is(43)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use eq_ord_tester::prelude::*;

	#[test]
	fn from_u64() {
		for i in 0..10 {
			let fixture = Element::from(i);
			fixture.is(i);
		}
	}

	#[test]
	fn from_str() {
		for expected in 0..10 {
			let fixture = Element::try_from(expected.to_string().as_str());
			fixture.unwrap().is(expected);
		}
	}

	#[test]
	fn invalid_from_str() {
		let fixture = Element::try_from("1  0");
		assert!(fixture.is_err())
	}

	#[test]
	fn partial_ord_test() {
		let piv = Element::from(42);
		let less = Element::from(41);
		let great = Element::from(43);
		let eq = Element::from(42);

		partial_simple_ord_test(piv, great, less, eq);

		let s = Element::from(42);
		let any = Element::any();

		assert_eq!(s.partial_cmp(&any), None);
		assert_eq!(any.partial_cmp(&s), None);
		assert_eq!(any.partial_cmp(&Element::any()), None);
	}

	#[test]
	fn partial_eq_test() {
		let x = Element::from(42);
		let y = Element::from(42);
		let z = Element::from(42);
		let not = Element::from(100);

		partial_ne_test(&x, &not);
		partial_reflexive_test(&x);
		partial_symmetric_test(&x, &y);
		partial_transitive_test(&x, &y, &z);

		let any = Element::any();
		assert!(!(x == any));
		assert!(!(any == x));
		assert!(!(any == any));
	}

	#[test]
	fn display() {
		let fixture = Element::from(42);
		let act = format!("{}", &fixture);
		assert_eq!(act, "42");

		let fixture = Element::Any;
		let act = format!("{}", &fixture);
		assert_eq!(act, "*");
	}

	#[test]
	fn debug() {
		let fixture = Element::from(42);
		let act = format!("{:?}", &fixture);
		assert_eq!("Specified(42)", act);

		let fixture = Element::any();
		let act = format!("{:?}", &fixture);
		assert_eq!("Any", act);
	}
}
