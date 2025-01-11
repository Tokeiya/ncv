use regex::Regex;
use std::cmp::Ordering;
mod test_helper;
mod version;

fn main() {
	let reg = Regex::new("(\\d+)\\.?(\\d+)\\.?(\\d+)").unwrap();

	let a = reg.captures("12.4.56").unwrap();

	for elem in a.iter().skip(1) {
		println!("{:?}", elem.unwrap().as_str());
	}
}
