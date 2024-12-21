use clap::Parser;

#[derive(Parser)]
#[command()]
pub struct CommandArgs {
	#[arg(short, long)]
	node: String,
}

impl CommandArgs {
	pub fn node(&self) -> &str {
		self.node.as_str()
	}
}
