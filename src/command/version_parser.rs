use combine;
use combine::easy::Error as EasyError;
use combine::easy::Stream as EasyStream;
use combine::{EasyParser, Parser};
use combine::{parser as psr, parser::char as chr};
use std::num::ParseIntError;

pub type Stream<'a> = EasyStream<&'a str>;

pub trait CharParser<'a, T>: EasyParser<Stream<'a>, Output = T> {}

fn digit() -> CharParser<Result<u64, ParseIntError>> {
	let tmp = chr::digit::<Stream>();
	let tmp = combine::many1::<String, Stream, _>(tmp);

	let tmp = tmp.map(|x| x.parse::<u64>());
	tmp
}
