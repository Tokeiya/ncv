use regex::Regex;
use std::fmt::{Debug, Display};
use std::sync::LazyLock;
use thiserror;
static REGEX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<revision>\d+)$").unwrap());

pub struct Version {
	major: u64,
	minor: u64,
	revision: u64,
}
