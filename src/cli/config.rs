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
        thread_count: Option<u8>,
    },
}

impl super::DoDoArgs for ConfigArgs {
    fn execute(&self) -> crate::Result<()> {
        match &self.commands {
            Some(ConfigCommands::Set { thread_count }) => {
                if thread_count.is_none() {
                    println!("{}", "No changes applied.".yellow());
                    return Ok(());
                }
                let mut config = Config::get(None)?;
                config.thread_count = thread_count.unwrap();
                match config.set(None) {
                    Ok(_) => println!("{}", "Updated config successfully!".green()),
                    Err(err) => eprintln!("Update failed with error: {}", err.to_string().red()),
                }
            }
            None => {
                let config = Config::get(None)?;
                println!(
                    "{}\nthread_count: {}\n{}",
                    "DoDo Config Start".yellow().underline(),
                    config.thread_count,
                    "DoDo Config END".yellow().underline()
                );
            }
        }
        Ok(())
    }
}
