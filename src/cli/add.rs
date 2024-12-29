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
        self.add_command(None)
    }
}

impl AddArgs {
    fn add_command(&self, path: Option<&str>) -> crate::Result<()> {
        let mut commands = Commands::get(path)?;
        commands.insert(self.name.clone(), self.command.clone());
        match commands.set(path) {
            Ok(_) => println!(
                "{}\n\tTry: {}",
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

#[cfg(test)]
mod tests {
    use fake::{
        faker::{lorem::en::Word, name::en::Name},
        Fake,
    };

    use super::*;

    macro_rules! assert_command_exists {
        ($entry: expr, $key: expr, $command: expr) => {
            assert!(
                $entry.is_some(),
                "Entry with doesn't exist with name: {}",
                $key
            );
            assert_eq!(
                *$entry.unwrap(),
                $command,
                "Entry with key with name: {}",
                $key
            );
        };
    }

    #[test]
    fn should_add_and_contain_new_commands() {
        let test_file = "dodo_test_add_command.json";
        let new_commands = (0..20)
            .map(|idx| AddArgs {
                name: Word().fake::<String>() + &idx.to_string(),
                command: format!("Export name=\"{}\"", Name().fake::<String>()),
            })
            .collect::<Vec<AddArgs>>();

        println!("{:#?}", new_commands);
        for arg in &new_commands {
            let _ = arg.add_command(Some(test_file));
        }

        let saved_command = Commands::get(Some(test_file)).unwrap();

        println!("{:#?}------", new_commands);
        for arg in &new_commands {
            assert_command_exists!(saved_command.get(&arg.name), arg.name, arg.command);
        }

        // cleanup
        std::fs::remove_file(test_file).unwrap();
    }
}
