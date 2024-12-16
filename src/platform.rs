use serde::Deserialize;
pub type StringValue = Option<String>;
pub type StrValue<'a> = Option<&'a str>;

#[derive(Deserialize)]
pub struct Node {
	pub(self) runtime: StringValue,
	pub(self) npm: StringValue,
}

#[derive(Deserialize)]
pub struct Platform {
	node: Node,
	pnpm: StringValue,
	yarn: StringValue,
}

impl Platform {}
