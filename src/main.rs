use clap::Parser;
use crate::models::cli::{Cli, Commands};

mod models;

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Go(args) => {
            println!("test1 {:?}", args.name);
        }
        Commands::List => {
            println!("test2");
        }
        Commands::Add(args) => {
            println!("{:?}", args)
        }
        Commands::Delete(args) => {
            println!("{:?}", args)
        }
    }
}