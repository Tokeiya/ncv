use regex::Regex;
use std::cmp::Ordering;
use std::sync::LazyLock;

mod test_helper;
mod version;

fn main() {
	println!("{}", u64::MAX);
}

fn foo() {
	let reg = Regex::new("^(\\d+)\\.(\\d+)\\.(\\d+)$").unwrap();

	static REG: LazyLock<Regex> =
		LazyLock::new(|| Regex::new("^(\\d+)\\.(\\d+)\\.(\\d+)$").unwrap());

	let a = reg.captures("12.4.56.");

	println!("{:?}", a);

	// for elem in a.iter().skip(1) {
	// 	println!("{:?}", elem.unwrap().as_str());
	// }
}

fn hoge() {
	static FOO: LazyLock<i32> = LazyLock::new(|| {
		println!("init");
		42i32
	});

	println!("{}", *FOO);
}
