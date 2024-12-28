use clap::{Args, Subcommand};

use crate::{data::Config, text::Color};

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
                match Config::new(*thread_count).set(None) {
                    Ok(_) => println!("Updated config successfully!"),
                    Err(err) => eprintln!("Update failed with error: {}", err),
                }
            }
            None => {
                let config = Config::get(None)?;
                println!(
                    "{}\nthread_count: {}\n{}",
                    "----DoDo Config----".yellow(),
                    config.thread_count,
                    "----END----".yellow()
                );
            }
        }
        Ok(())
    }
}
