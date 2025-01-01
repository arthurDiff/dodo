use clap::Args;

use crate::{
    data::Commands,
    text::{Color, Font},
};

// probably add something to specify as route/ path
#[derive(Debug, Args)]
pub struct AddArgs {
    /// name of the command
    name: String,
    /// command to be mapped to the name
    command: String,
    #[arg(short, long, default_value_t = false)]
    path: bool,
}

impl super::DoDoArgs for AddArgs {
    fn execute(&self) -> crate::Result<()> {
        self.add_commands(None)
    }
}

impl AddArgs {
    fn add_commands(&self, path: Option<&str>) -> crate::Result<()> {
        let mut commands = Commands::get(path)?;
        commands.insert(self.name.clone(), self.command.clone());
        match commands.set(path) {
            Ok(_) => println!(
                "{} {}",
                "New command has been added (Try):".green(),
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
                path: false,
            })
            .collect::<Vec<AddArgs>>();

        for arg in &new_commands {
            let _ = arg.add_commands(Some(test_file));
        }

        let saved_command =
            Commands::get(Some(test_file)).expect("Failed saving command for adding new command");

        for arg in &new_commands {
            assert_command_exists!(saved_command.get(&arg.name), arg.name, arg.command);
        }

        std::fs::remove_file(test_file).unwrap_or_else(|e| {
            panic!(
                "Expected to cleanup testfile: {}, but got: {}",
                test_file, e
            )
        });
    }
}
