use clap::{Args, Subcommand};

use crate::{
    data::Config,
    text::{Color, Font},
};

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    commands: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    Set {
        // Number of threads for concurrent runs
        #[arg(short, long)]
        thread_count: u8,
    },
}

impl super::DoDoArgs for ConfigArgs {
    fn execute(&self) -> crate::Result<()> {
        match &self.commands {
            Some(ConfigCommands::Set { thread_count }) => {
                let mut config = Config::get(None)?;
                config.thread_count = *thread_count;
                match config.set(None) {
                    Ok(_) => println!("{}", "Updated config successfully!".green()),
                    Err(err) => eprintln!("Update failed with error: {}", err.to_string().red()),
                }
            }
            None => {
                let config = Config::get(None)?;
                println!(
                    "{}\nthread_count: {}\n{}",
                    "DoDo Config".yellow().bold().underline(),
                    config.thread_count,
                    "----END----".yellow()
                );
            }
        }
        Ok(())
    }
}
