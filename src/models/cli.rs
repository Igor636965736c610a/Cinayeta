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
#[command(help_template = "{usage}\n\x1b[1mExample:\x1b[0m  yeta go your_saved_command\n\n{all-args}")]
pub struct GoArgs {
    /// Specific command name to run immediately [OPTIONAL]
    pub name: Option<String>,
    /// Overwrite executable. Example: ("dotnet", "git") or full path to .exe [OPTIONAL]
    pub exe: Option<Exe>
}

#[derive(Debug, Args)]
#[command(help_template = "{usage}\n\x1b[1mExample:\x1b[0m  yeta add \"my_command\" \"git clean\" --args \"-dfx -i\"\n\n{all-args}")]
pub struct AddArgs {
    /// Specific unique command name
    pub name: String,
    /// Executable ("dotnet", "git") or full path to .exe
    pub exe: Exe,
    /// Arguments for your exe [OPTIONAL]
    #[arg(long = "args")]
    pub args: Option<String>
}

#[derive(Debug, Clone)]
pub enum Exe {
    Program(String),
    Path(PathBuf),
}

#[derive(Debug, Args)]
#[command(help_template = "{usage}\n\x1b[1mExample:\x1b[0m  yeta delete my_command\n\n{all-args}")]
pub struct DeleteArgs {
    /// Specific command name [OPTIONAL]
    pub name: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs specific or selected command  [OPTIONAL]<NAME> [OPTIONAL]<EXE>
    Go(GoArgs),
    /// List all commands.
    List,
    /// Add command <NAME> <EXE> [OPTIONAL]--args <ARGS>
    Add(AddArgs),
    /// Delete command  [OPTIONAL]<NAME>
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