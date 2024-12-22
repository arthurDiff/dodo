use clap::{Parser, Subcommand};
// internal
use about::AboutArgs;
use delete::DeleteArgs;
use list::ListArgs;
use run::RunArgs;
use save::SaveArgs;

mod about;
mod delete;
mod list;
mod run;
mod save;
//https://docs.rs/clap/latest/clap/_derive/index.html
#[derive(Debug, Parser)]
#[command(name = "dodo")]
#[command(version)]
pub struct DoDo {
    #[command(subcommand)]
    dodo: DoDoCommands,
}

#[derive(Debug, Subcommand)]
enum DoDoCommands {
    /// Run a command or commands
    Run(RunArgs),
    /// Save new command
    Save(SaveArgs),
    /// Delete command
    Delete(DeleteArgs),
    /// List saved commands
    List(ListArgs),
    /// DoDo dance
    About(AboutArgs),
}

impl DoDo {
    pub fn run() -> Self {
        Self::parse()
    }
}

pub trait DoDoArgs {
    #[allow(dead_code)]
    fn execute(&self) -> crate::Result<()>;
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::DoDo;

    #[test]
    fn cli_runs_right() {
        DoDo::command().debug_assert();
    }
}
