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

#[derive(Debug, Parser)]
#[command(name = "dodo")]
#[command(version, about)]
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

// trait DoDoArgs {}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::DoDo;

    #[test]
    fn cli_runs_right() {
        DoDo::command().debug_assert();
    }
}
