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
    /// Overwrite executable. Example: ("dotnet run", "git commit") or full path to .exe [OPTIONAL]
    #[arg(long = "exe", short = "e")]
    pub exe: Option<Exe>
}

#[derive(Debug, Args)]
#[command(help_template = "{usage}\n\x1b[1mExample:\x1b[0m  yeta add \"my_command\" \"git clean\" --args \"-dfx -i\"\n\n{all-args}")]
pub struct AddArgs {
    /// Specific unique command name
    pub name: String,
    /// Executable ("dotnet run", "git commit") or full path to .exe
    pub exe: Exe,
    /// Arguments for your exe [OPTIONAL]
    #[arg(long = "args")]
    pub args: Option<String>
}

#[derive(Debug, Args)]
#[command(help_template = "{usage}\n\x1b[1mExample:\x1b[0m  yeta delete my_command\n\n{all-args}")]
pub struct DeleteArgs {
    /// Specific command name [OPTIONAL]
    pub name: Option<String>
}

#[derive(Debug, Args)]
#[command(help_template = "{usage}\n\x1b[1mExample:\x1b[0m  yeta update \"my_command\" \"git reset\" --args \"--hard -f\"\n\n{all-args}")]
pub struct UpdateArgs {
    /// Specific unique command name to override
    pub name: String,
    /// Executable ("dotnet run", "git commit") or full path to .exe to override [OPTIONAL]
    pub exe: Option<Exe>,
    /// Arguments for your exe to override [OPTIONAL]
    #[arg(long = "args")]
    pub args: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs specific or selected command  [OPTIONAL]<NAME> [OPTIONAL]-e <EXE>
    Go(GoArgs),
    /// List all commands.
    List,
    /// Add command <NAME> <EXE> [OPTIONAL]--args <ARGS>
    Add(AddArgs),
    /// Update command <NAME> [OPTIONAL]<EXE> [OPTIONAL]--args <ARGS>
    Update(UpdateArgs),
    /// Delete command  [OPTIONAL]<NAME>
    Delete(DeleteArgs)
}

#[derive(Debug, Clone)]
pub enum Exe {
    Program(String),
    Path(PathBuf),
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