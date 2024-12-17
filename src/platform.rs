use super::platform_load_error::Error;
use serde::Deserialize;
use serde_json::Value;
use std::io::Read;
pub type StringValue = Option<String>;
pub type StrValue<'a> = Option<&'a str>;

#[derive(Deserialize)]
pub struct Node {
	pub(self) runtime: String,
	pub(self) npm: StringValue,
}

#[derive(Deserialize)]
pub struct Platform {
	node: Node,
	pnpm: StringValue,
	yarn: StringValue,
}

impl Platform {

    pub fn from_read(read:impl Read)->Result<Platform,Error>{
		let value:Value=serde_json::from_reader(read)?;

		let node=value.get("node").ok_or(Error::EntryNotFound("node".to_string()))?;
		
		let tmp=node.get("runtime").ok_or(Error::EntryNotFound("runtime".to_string()))?;
		let runtime=tmp.as_str().ok_or(Error::UnexpectedValueType("Null".to_string()))?.to_string();
		
		let npm=node.get("npm").ok_or(Error::EntryNotFound("npm".to_string()))?.
			as_str().map(|x|x.to_string());
		
		let pnpm=value.get("pnpm").ok_or(Error::EntryNotFound("pnpm".to_string()))?
			.as_str().map(|x|x.to_string());
		
		let yarn=value.get("yarn").ok_or(Error::EntryNotFound("yarn".to_string()))?
			.as_str().map(|x|x.to_string());
		
		let node=Node{runtime,npm};
		
		Ok(Platform{node,pnpm,yarn})
		
	}


	pub fn node_runtime(&self)->&'_ str{
		self.node.runtime.as_str()
	}

	pub fn node_npm(&self)->StrValue<'_>{
		match &self.node.npm {
			None => None,
			Some(x) => Some(x.as_str())
		}
	}

	pub fn pnpm(&self)->StrValue<'_>{
		match &self.pnpm {
			None => None,
			Some(s) => Some(s.as_str())
		}
	}

	pub fn yarn(&self)->StrValue<'_>{
		match &self.yarn {
			None => None,
			Some(s) => Some(s.as_str())
		}
	}
}

#[cfg(test)]
mod test{
	use super::*;
	use crate::platform_load_error::Error;
	use std::fs::File;


	#[test]
	fn from_read_test() {
		let fixture=Platform::from_read(File::open("./test_artifacts/valid.json").unwrap()).unwrap();
		assert_eq!(fixture.node.npm,None);
		assert_eq!(fixture.node.runtime,"23.4.0");
		assert_eq!(fixture.pnpm.unwrap(),"9.15.0");
		assert_eq!(fixture.yarn,None);
	}
	#[test]
	fn invalid_format() {
		let fixture=Platform::from_read(File::open("./test_artifacts/invalid_format.txt").unwrap());
		assert!(matches!(fixture,Err(Error::Serde(_))));
	}

	#[test]
	fn shortage() {
		let fixture=Platform::from_read(File::open("./test_artifacts/shortage.json").unwrap());
		assert!(matches!(fixture,Err(Error::EntryNotFound(_))));
	}

	#[test]
	fn node_runtime_test(){
		let fixture=Platform::from_read(File::open("./test_artifacts/full_defined.json").unwrap()).unwrap();
		assert_eq!(fixture.node_runtime(),"23.4.0");
	}

	#[test]
	fn node_npm_test() {
		let fixture=Platform::from_read(File::open("./test_artifacts/full_defined.json").unwrap()).unwrap();
		assert_eq!(fixture.node_npm().unwrap(),"10.9.2");
	}

	#[test]
	fn pnpm_test(){
		let fixture=Platform::from_read(File::open("./test_artifacts/full_defined.json").unwrap()).unwrap();
		assert_eq!(fixture.pnpm().unwrap(),"9.15.0");
	}

	#[test]
	fn yarn_test(){
		let fixture=Platform::from_read(File::open("./test_artifacts/full_defined.json").unwrap()).unwrap();
		assert_eq!(fixture.yarn().unwrap(),"4.5.3");
	}
}



