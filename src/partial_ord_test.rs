#[cfg(test)]
pub mod partial_ord_tests {
	use std::cmp::Ordering;

	pub fn greater_test<T: PartialOrd>(pivot: &T, greater: &T) {
		dbg!(pivot.partial_cmp(greater));

		assert!(matches!(
			pivot.partial_cmp(greater).unwrap(),
			Ordering::Less
		));
		assert!(matches!(
			greater.partial_cmp(pivot).unwrap(),
			Ordering::Greater
		));
	}

	pub fn greater_than_equal_test<T: PartialOrd>(pivot: &T, greater: &T, equal: &T) {
		assert!(matches!(
			pivot.partial_cmp(greater).unwrap(),
			Ordering::Less
		));
		assert!(matches!(
			greater.partial_cmp(pivot).unwrap(),
			Ordering::Greater
		));
		assert!(matches!(
			greater.partial_cmp(pivot).unwrap(),
			Ordering::Greater
		));

		assert!(matches!(pivot.partial_cmp(equal).unwrap(), Ordering::Equal));
		assert!(matches!(equal.partial_cmp(equal).unwrap(), Ordering::Equal));
	}

	pub fn less_test<T: PartialOrd>(pivot: &T, less: &T) {
		assert!(matches!(
			pivot.partial_cmp(less).unwrap(),
			Ordering::Greater
		));
		assert!(matches!(less.partial_cmp(pivot).unwrap(), Ordering::Less));
	}

	pub fn less_than_equal_test<T: PartialOrd>(pivot: &T, less: &T, equal: &T) {
		assert!(matches!(
			pivot.partial_cmp(less).unwrap(),
			Ordering::Greater
		));
		assert!(matches!(less.partial_cmp(pivot).unwrap(), Ordering::Less));

		assert!(matches!(pivot.partial_cmp(equal).unwrap(), Ordering::Equal));
		assert!(matches!(equal.partial_cmp(equal).unwrap(), Ordering::Equal));
	}

	pub fn partial_ord_test<T: PartialOrd>(pivot: &T, less: &T, greater: &T, equal: &T) {
		greater_test(pivot, greater);
		greater_than_equal_test(pivot, greater, equal);

		less_test(pivot, less);
		less_than_equal_test(pivot, less, equal);
	}
}

#[cfg(test)]
mod tests {
	use super::partial_ord_tests::*;

	#[test]
	fn greater() {
		greater_test(&42, &43);
	}

	#[test]
	#[should_panic]
	fn invalid_greater() {
		greater_test(&42, &42);
	}

	#[test]
	fn greater_than_equal() {
		greater_than_equal_test(&42, &43, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_than_equal_a() {
		greater_than_equal_test(&42, &42, &42)
	}

	#[test]
	#[should_panic]
	fn invalid_than_equal_b() {
		greater_than_equal_test(&42, &43, &41)
	}

	#[test]
	fn less() {
		less_test(&42, &41);
	}

	#[test]
	#[should_panic]
	fn invalid_less() {
		less_test(&42, &42);
	}

	#[test]
	fn less_than_equal() {
		less_than_equal_test(&42, &41, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_less_than_equal_a() {
		less_than_equal_test(&42, &42, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_less_than_equal_b() {
		less_than_equal_test(&42, &41, &43);
	}

	#[test]
	fn partial_ord() {
		partial_ord_test(&42, &41, &43, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_partial_ord_a() {
		partial_ord_test(&42, &42, &43, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_partial_ord_b() {
		partial_ord_test(&42, &41, &42, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_partial_ord_c() {
		partial_ord_test(&42, &41, &43, &41);
	}
}
