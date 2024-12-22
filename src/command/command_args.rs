use clap::Parser;

#[derive(Parser)]
#[command()]
pub struct CommandArgs {
	#[arg(short, long)]
	version: String,
	#[arg(short, long)]
	target: String,
}

impl CommandArgs {
	pub fn version(&self) -> &str {
		self.version.as_str()
	}

	pub fn target(&self) -> &str {
		self.target.as_str()
	}
}
