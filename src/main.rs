mod test_helper;
mod version;

fn main() {
	let piv: Option<u32> = None;
	let spec: Option<u32> = 42.into();

	let a = piv >= spec;
	let b = piv <= spec;
}
