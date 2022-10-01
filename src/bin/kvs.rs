use crate::lib::{Cli, Commands};
use clap::Parser;
use std::process::exit;

mod lib;

fn main() {
    let app = Cli::parse();
    match &app.command {
        Commands::Get {..} | Commands::Set {..} | Commands::Rm {..} => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
