use clap::{Parser, Subcommand};
// internal
use about::AboutArgs;
use add::AddArgs;
use config::ConfigArgs;
use list::ListArgs;
use remove::RemoveArgs;
use run::RunArgs;

mod about;
mod add;
mod config;
mod list;
mod remove;
mod run;

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
    /// Run command(s)
    Run(RunArgs),
    /// Save new command(s)
    Add(AddArgs),
    /// Delete command(s)
    Remove(RemoveArgs),
    /// List saved commands
    List(ListArgs),
    /// Read or Write Config
    Config(ConfigArgs),
    /// DoDo dance
    About(AboutArgs),
}
//https://github.com/clap-rs/clap/discussions/3715
impl DoDo {
    pub fn run() -> Self {
        Self::parse()
    }

    pub fn execute(&self) -> crate::Result<()> {
        match &self.dodo {
            DoDoCommands::Run(run_args) => run_args.execute(),
            DoDoCommands::Add(add_args) => add_args.execute(),
            DoDoCommands::Remove(remove_args) => remove_args.execute(),
            DoDoCommands::List(list_args) => list_args.execute(),
            DoDoCommands::Config(config_args) => config_args.execute(),
            DoDoCommands::About(about_args) => about_args.execute(),
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
