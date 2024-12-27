use clap::{Args, Subcommand};

use crate::data::Config;

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    commands: ConfigCommands,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    Get,
    Set {
        // Number of threads for concurrent runs
        #[arg(short, long, default_value_t = 4)]
        thread_count: u8,
    },
}

impl super::DoDoArgs for ConfigArgs {
    fn execute(&self) -> crate::Result<()> {
        match self.commands {
            ConfigCommands::Get => println!("{}", Config::get(None)?),
            ConfigCommands::Set { thread_count } => match Config::new(thread_count).set(None) {
                Ok(_) => println!("Updated config successfully!"),
                Err(err) => println!("Update failed with error:\n{}", err),
            },
        }
        Ok(())
    }
}
