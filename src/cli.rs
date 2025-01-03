use clap::{Parser, Subcommand};
// internal
use add::AddArgs;
use bark::BarkArgs;
use list::ListArgs;
use remove::RemoveArgs;
use run::RunArgs;

mod add;
mod bark;
mod list;
mod remove;
mod run;

//https://docs.rs/clap/latest/clap/_derive/index.html
#[derive(Debug, Parser)]
#[command(name = "dodo")]
#[command(version)]
pub struct DoDo {
    #[command(subcommand)]
    dodo: Option<DoDoCommands>,
    #[command(flatten)]
    run_args: RunArgs,
}

#[derive(Debug, Subcommand)]
enum DoDoCommands {
    /// Run command(s)
    Run(RunArgs),
    /// Save new command(s)
    Add(AddArgs),
    /// Delete command(s)
    Remove(RemoveArgs),
    /// List saved commands
    List(ListArgs),
    /// DoDo bark
    Bark(BarkArgs),
}

//https://github.com/clap-rs/clap/discussions/3715
impl DoDo {
    pub fn run() -> Self {
        Self::parse()
    }

    pub fn execute(&self) -> crate::Result<()> {
        if let Some(cmd) = &self.dodo {
            match cmd {
                DoDoCommands::Run(run_args) => run_args.execute(),
                DoDoCommands::Add(add_args) => add_args.execute(),
                DoDoCommands::Remove(remove_args) => remove_args.execute(),
                DoDoCommands::List(list_args) => list_args.execute(),
                DoDoCommands::Bark(bark_args) => bark_args.execute(),
            }
        } else {
            self.run_args.execute()
        }
    }
}

trait DoDoArgs {
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
