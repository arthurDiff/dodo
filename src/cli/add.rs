use clap::Args;

use crate::{
    data::Commands,
    text::{Color, Font},
};

#[derive(Debug, Args)]
pub struct AddArgs {
    /// name of the command
    #[arg(short, long)]
    name: String,
    /// command to be mapped to the name
    #[arg(short, long)]
    command: String,
}

impl super::DoDoArgs for AddArgs {
    fn execute(&self) -> crate::Result<()> {
        let mut commands = Commands::get(None)?;
        commands.insert(self.name.clone(), self.command.clone());
        match commands.set(None) {
            Ok(_) => println!(
                "{}\n    Try: {}",
                "New command has been added:".green(),
                format!("dodo run {}", self.name).bold()
            ),
            Err(err) => eprintln!(
                "Failed adding new command with error: {}",
                err.to_string().red()
            ),
        }
        Ok(())
    }
}
