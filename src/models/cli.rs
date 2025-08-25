use std::path::PathBuf;
use std::str::FromStr;
use clap::{Args, Parser, Subcommand};

/*
    \x1b[1m -> bold
    \x1b[4m -> cursive
    \x1b[0m -> reset style
 */

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Args)]
pub struct GoArgs {
    /// Specific command name to run immediately
    pub name: Option<String>,
    /// Executable ("dotnet", "git") or full path to .exe
    pub exe: Option<Exe>
}

#[derive(Debug, Args)]
#[command(help_template = "{usage}\n\n\x1b[1;4mExample:\x1b[0m git clean --args -dfx\n\n{all-args}")]
pub struct AddArgs {
    /// Specific unique command name
    pub name: String,
    /// Executable ("dotnet", "git") or full path to .exe
    pub exe: Exe,
    /// Arguments for your exe
    #[arg(long = "args", num_args = 1.., allow_hyphen_values = true)]
    pub args: Option<Vec<String>>
}

#[derive(Debug, Clone)]
pub enum Exe {
    Program(String),
    Path(PathBuf),
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Specific command name
    pub name: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs specific or selected command  [OPTIONAL]--name <NAME> [OPTIONAL]--exe <EXE>
    Go(GoArgs),
    /// List all commands.
    List,
    /// Add command  --name <NAME> --exe <EXE> [OPTIONAL]--args <ARGS>
    Add(AddArgs),
    /// Delete command  [OPTIONAL]--name <NAME>
    Delete(DeleteArgs)
}

impl FromStr for Exe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        if s.contains('/') || s.contains('\\') {
            Ok(Exe::Path(path))
        } else {
            Ok(Exe::Program(s.to_string()))
        }
    }
}