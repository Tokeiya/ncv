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
