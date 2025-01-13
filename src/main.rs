use regex::Regex;
use std::cmp::Ordering;
use std::sync::LazyLock;

mod test_helper;
mod version;

fn main() {
	static MAJOR: LazyLock<regex::Regex> =
		LazyLock::new(|| regex::Regex::new(r"^(\d+)(\.\*)?$").unwrap());

	static MINOR: LazyLock<regex::Regex> =
		LazyLock::new(|| regex::Regex::new(r"^(\d+)\.(\d+)(\.\*)?$").unwrap());

	static PATCH: LazyLock<regex::Regex> =
		LazyLock::new(|| regex::Regex::new(r"^(\d+)\.(\d+)\.(\d+)$").unwrap());

	let a = MINOR.captures("12").unwrap();
	println!("{:?}", a);

	println!("{}", a.get(1).unwrap().as_str());
	println!("{}", a.get(2).unwrap().as_str());
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
