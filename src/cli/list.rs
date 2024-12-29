use clap::Args;

use crate::text::{Color, Font};

#[derive(Debug, Args)]
pub struct ListArgs;

impl super::DoDoArgs for ListArgs {
    fn execute(&self) -> crate::Result<()> {
        let cmds = self.list_commands_as_lines()?;
        println!(
            "{}\n{}\n{}",
            "DoDo Commands".yellow().bold().underline(),
            cmds,
            "----END----".yellow()
        );
        Ok(())
    }
}

impl ListArgs {
    fn list_commands_as_lines(&self) -> crate::Result<String> {
        let commands = crate::data::Commands::get(None)?;
        let mut l_str = String::new();
        for (k, v) in commands.iter() {
            l_str += &format!("\n{k} : {v}");
        }
        Ok(l_str)
    }
}
