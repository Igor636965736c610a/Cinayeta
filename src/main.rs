use clap::Parser;
use crate::Cli::args::{Cli, Commands};

mod Cli;
mod shell;
mod Core;

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
        Commands::Update(args) => {
            println!("{:?}", args)
        }
    }
}