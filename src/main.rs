mod test_helper;
mod version;
mod version_element;

use combine::{self as cmb, parser::char as chr};
use std::io;

use combine::{EasyParser, stream::easy::Stream as EasyStream};

fn read_line() -> io::Result<String> {
	let mut buff = String::new();

	io::stdin().read_line(&mut buff)?;
	Ok(buff)
}

fn digit_parser<I: cmb::Stream<Token = char>>() -> impl cmb::Parser<I, Output = String> {
	let digit = chr::digit::<I>();
	let digits = cmb::many1::<String, _, _>(digit);
	digits
}

fn foo<'a>() -> impl EasyParser<EasyStream<&'a str>, Output = String> {
	let digit = chr::digit();
	cmb::many1(digit)
}

fn main() {
	let input = read_line().unwrap();
	let stream = cmb::easy::Stream(input.as_str());
	let output = foo().easy_parse(stream);

	println!("{:?}", output)
}
