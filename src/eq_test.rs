#[cfg(test)]
pub mod equality_tests {

	pub fn equality_test<T: PartialEq>(x: &T, y: &T, z: &T, not_eq: &T) {
		reflexive_test(x, y);
		symmetry_test(x);
		transitive_test(x, y, z);
		not_equality_test(x, not_eq);
	}
	pub fn reflexive_test<T: PartialEq>(x: &T, y: &T) {
		assert!(x == y);
		assert!(y == x);
	}

	pub fn symmetry_test<T: PartialEq>(x: &T) {
		assert!(x.eq(x));
	}

	pub fn transitive_test<T: PartialEq>(x: &T, y: &T, z: &T) {
		assert!(x == y);
		assert!(y == z);
		assert!(x == z);
	}

	pub fn not_equality_test<T: PartialEq>(x: &T, other: &T) {
		assert!(x != other)
	}
}

#[cfg(test)]
pub mod test {
	use super::equality_tests::*;
	use mockall::mock;

	mock! {
		Value{}

		impl PartialEq for Value{
			fn eq(&self, other: &Self) -> bool;
		}
	}

	#[test]
	fn equal_test() {
		equality_test(&42, &42, &42, &43);
	}

	#[test]
	fn reflexive() {
		reflexive_test(&42, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_reflexive() {
		reflexive_test(&42, &10)
	}

	#[test]
	fn symmetry() {
		symmetry_test(&42)
	}

	#[test]
	#[should_panic]
	fn invalid_symmetry() {
		let mut mock = MockValue::new();
		mock.expect_eq().times(1).return_const(false);

		symmetry_test(&mock)
	}

	#[test]
	fn transitive() {
		transitive_test(&42, &42, &42);
	}

	#[test]
	#[should_panic]
	fn invalid_transitive() {
		transitive_test(&42, &43, &42);
	}

	#[test]
	fn not_eq_test() {
		not_equality_test(&42, &43)
	}

	#[test]
	#[should_panic]
	fn invalid_not_eq_test() {
		not_equality_test(&42, &42)
	}
}
