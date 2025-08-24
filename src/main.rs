use clap::Parser;
use crate::models::cli::{Cli, Commands};

mod models;

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Invoke(args) => {
            println!("test1 {:?}", args.name);
        }
        Commands::List => {
            println!("test2");
        }
    }
}