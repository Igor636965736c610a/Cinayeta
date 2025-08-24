use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Args)]
pub struct InvokeArgs {
    pub name: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Invoke(InvokeArgs),
    List
}