use clap::Args;

use crate::{
    data::Commands,
    text::{Color, Font},
};

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Names of space seperated commands to run
    names: Vec<String>,
    /// Run commands asynchronously (Default false)
    #[arg(short = 'a', long, default_value_t = false)]
    run_async: bool,
}

impl super::DoDoArgs for RunArgs {
    fn execute(&self) -> crate::Result<()> {
        if self.names.is_empty() {
            println!(
                "Requires at least one command to run!\nIf you need help try: {} or {}",
                "dodo list".yellow(),
                "dodo help".yellow()
            );
            return Ok(());
        }
        if self.run_async {
            self.run_commands_async(None)
        } else {
            self.run_commands_sync(None)
        }
    }
}

impl RunArgs {
    fn run_commands_async(&self, _path: Option<&str>) -> crate::Result<()> {
        todo!()
    }

    fn run_commands_sync(&self, path: Option<&str>) -> crate::Result<()> {
        let cmds = Commands::get(path)?;
        for n in &self.names {
            if let Some(cmd) = cmds.get(n) {
                Self::run_command(cmd)?;
            } else {
                println!("DoDo commands doesn't contain: {}", n.yellow().bold());
            }
        }
        Ok(())
    }

    fn run_command(cmd_str: &str) -> crate::Result<()> {
        let cmd = cmd_str.split_whitespace().collect::<String>();
        println!("{}", cmd);

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
