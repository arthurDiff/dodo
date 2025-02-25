use clap::Args;

use crate::{
    data::{get_relative_to_bin_as_pathbuf, Commands},
    text::{Color, Font},
};

// probably add something to specify as route/ path
#[derive(Debug, Args)]
pub(crate) struct AddArgs {
    /// name of the command
    name: String,
    /// command to be mapped to the name
    command: String,
    /// indicate if command is path (supports relative or absolute)
    #[arg(short = 'p', long = "path", default_value_t = false)]
    is_path: bool,
    /// copy specified path to binary folder
    #[arg(short = 'c', long, default_value_t = false)]
    copy_file: bool,
}

impl super::DoDoArgs for AddArgs {
    fn execute(&self) -> crate::Result<()> {
        self.add_commands(None)
    }
}

impl AddArgs {
    fn add_commands(&self, path: Option<&str>) -> crate::Result<()> {
        let mut commands = Commands::get(path)?;
        let new_cmd = if self.is_path {
            match std::path::absolute(&self.command) {
                Ok(ab_path) => {
                    let mut file_path = ab_path;
                    let file_name = file_path
                        .file_name()
                        .expect("Failed should have contained filename from canonicalization.")
                        .to_str()
                        .expect(
                            "Failed to stringify file name should be valid from canonicalization.",
                        );
                    if self.copy_file {
                        if let Ok(store_path) = get_relative_to_bin_as_pathbuf(file_name) {
                            match std::fs::copy(&file_path, &store_path) {
                                Ok(_) => {
                                    println!("File {} Copied Successfully", file_name.green());
                                    file_path = store_path;
                                }
                                Err(err) => {
                                    eprintln!(
                                        "Failed to copy file at {} with error: {}",
                                        file_path.display().to_string().yellow(),
                                        err.to_string().red()
                                    );
                                }
                            }
                        } else {
                            println!(
                                "Failed to copy file at {}, will use provided path as destination.",
                                file_path.display().to_string().red()
                            );
                        };
                    }
                    // keeping unix path def
                    file_path.display().to_string().replace("\\", "/")
                }
                Err(err) => {
                    eprintln!(
                        "Failed generating absolute path with error: {}",
                        err.to_string().red()
                    );
                    return Ok(());
                }
            }
        } else {
            self.command.clone()
        };
        commands.insert(self.name.clone(), new_cmd);
        match commands.set(path) {
            Ok(_) => {
                println!(
                    "{} {}",
                    "New command has been added (Try):".green(),
                    format!("dodo run {}", self.name).underline()
                );
            }
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
                is_path: false,
                copy_file: false,
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

        std::fs::remove_file(crate::data::get_relative_to_bin(test_file).unwrap()).unwrap_or_else(
            |e| {
                panic!(
                    "Expected to cleanup testfile: {}, but got: {}",
                    test_file, e
                )
            },
        );
    }

    #[test]
    fn should_add_relative_path_correctly() {
        let test_file = "dodo_test_add_command_2.json";
        let path_arg = AddArgs {
            command: "./".to_owned(),
            name: "test".to_owned(),
            is_path: true,
            copy_file: false,
        };

        path_arg
            .add_commands(Some(test_file))
            .expect("Failed to create and add command to test file");

        let cmds = Commands::get(Some(test_file)).expect("Failed to get test command file");
        let test_ab_path = cmds
            .get("test")
            .expect("Failed to get newly added path command");

        assert!(std::path::Path::new(test_ab_path).is_absolute());

        std::fs::remove_file(crate::data::get_relative_to_bin(test_file).unwrap()).unwrap_or_else(
            |e| {
                panic!(
                    "Expected to cleanup testfile: {}, but got: {}",
                    test_file, e
                )
            },
        );
    }
}
