use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kvs", version, about, author)]
struct Cli {
	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand)]
enum Commands {
	/// Set a new key-value
	Set {
		/// String key
		#[arg(required = true)]
		key: String,
		/// String value
		#[arg(required = true)]
		value: String
	},
	/// Get the value of a key
	Get {
		/// String key to retrieve the value
		#[arg(required = true)]
		key: String
	},
	/// Remove a key
	Rm {
		/// String key to remove
		#[arg(required = true)]
		key: String
	},
}

#[warn(unused_variables)]
fn main() {
	Cli::parse();
    
	unimplemented!("unimplemented")
}
