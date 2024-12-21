use regex::Regex;
use std::fmt::{Debug, Display, Formatter};
use std::sync::LazyLock;
use thiserror;
static REGEX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<revision>\d+)$").unwrap());

pub struct NodeVersion {
	major: String,
	minor: String,
	revision: String,
}

impl Debug for NodeVersion {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}.{}", &self.major, &self.minor, &self.revision)
	}
}

impl Display for NodeVersion {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}.{}", &self.major, &self.minor, &self.revision)
	}
}

impl NodeVersion {
	pub fn new(value: String) -> Option<Self> {
		let cap = REGEX.captures(&value)?;
		Some(NodeVersion {
			major: cap["major"].to_string(),
			minor: cap["minor"].to_string(),
			revision: cap["revision"].to_string(),
		})
	}

	pub fn major(&self) -> &str {
		self.major.as_str()
	}

	pub fn minor(&self) -> &str {
		self.minor.as_str()
	}

	pub fn revision(&self) -> &str {
		self.revision.as_str()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn valid_new() {
		let fixture = NodeVersion::new("23.0.52".to_string()).unwrap();
		assert_eq!(fixture.major.as_str(), "23");
		assert_eq!(fixture.minor.as_str(), "0");
		assert_eq!(fixture.revision.as_str(), "52")
	}

	#[test]
	fn invalid_new() {
		assert!(NodeVersion::new("23".to_string()).is_none())
	}

	#[test]
	fn major() {
		let fixture = NodeVersion::new("23.0.52".to_string()).unwrap();
		assert_eq!(fixture.major(), "23")
	}

	#[test]
	fn minor() {
		let fixture = NodeVersion::new("23.0.52".to_string()).unwrap();
		assert_eq!(fixture.minor(), "0");
	}

	#[test]
	fn revision() {
		let fixture = NodeVersion::new("23.0.52".to_string()).unwrap();
		assert_eq!(fixture.revision(), "52");
	}

	#[test]
	fn display() {
		let fixture = NodeVersion::new("23.0.52".to_string()).unwrap();
		assert_eq!(format!("{}", fixture), "23.0.52")
	}

	#[test]
	fn debug() {
		let fixture = NodeVersion::new("23.0.52".to_string()).unwrap();
		assert_eq!(format!("{:?}", fixture), "23.0.52")
	}
}
