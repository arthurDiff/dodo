use clap::Args;

use crate::text::{Color, Font};

// add optional arg to export command file
#[derive(Debug, Args)]
pub(crate) struct ListArgs;

impl super::DoDoArgs for ListArgs {
    fn execute(&self) -> crate::Result<()> {
        let cmds = self.list_commands_as_lines()?;
        println!(
            "{}\n{}{}",
            "DoDo Commands Start".yellow().underline(),
            cmds,
            "DoDo Commands END".yellow().underline()
        );
        Ok(())
    }
}

impl ListArgs {
    fn list_commands_as_lines(&self) -> crate::Result<String> {
        let commands = crate::data::Commands::get(None)?;
        let mut l_str = String::new();
        for (k, v) in commands.iter() {
            l_str += &format!("{k} : {v}\n");
        }
        Ok(l_str)
    }
}
