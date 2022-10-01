// Clap tutorial -- https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Gets the value against a given key
    Get {
        #[arg(name = "key", value_name = "key")]
        key: String,
    },
    /// Sets the value against a given key
    Set {
        #[arg(name = "key", value_name = "key")]
        key: String,

        #[arg(name = "value")]
        value: String,
    },
    /// Removes the given key along with its value
    Rm {
        #[arg(name = "key")]
        key: String,
    },
}
