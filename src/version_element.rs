use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter};

pub const ANY_SIGN: char = '*';

pub enum Element {
	Any,
	Specified(u64),
}

impl Debug for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Display for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl PartialEq for Element {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl PartialOrd for Element {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}

	pub fn any() -> Self {
		todo!()
	}
}

impl From<u64> for Element {
	fn from(value: u64) -> Self {
		todo!()
	}
}

impl From<&str> for Element {
	fn from(value: &str) -> Self {
		todo!()
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl Element {
		pub fn is_any(&self) {
			assert!(matches!(self, Element::Any))
		}

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
	#[should_panic]
	fn invalid_is_any() {
		let fixture = Element::Specified(42);
		fixture.is_any()
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

// #[cfg(test)]
// mod tests {
// 	use super::*;
//
// 	#[test]
// 	fn any_test() {
// 		let fixture = Element::any();
// 		fixture.is_any()
// 	}
//
// 	#[test]
// 	fn from_u64() {
// 		for expected in 0..100 {
// 			let fixture = Element::from(expected);
// 			fixture.is(expected)
// 		}
// 	}
// }
