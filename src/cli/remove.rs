use clap::Args;

use crate::{
    data::Commands,
    text::{Color, Font},
};

#[derive(Debug, Args)]
pub(crate) struct RemoveArgs {
    #[arg(num_args=1.., use_value_delimiter=true, value_delimiter=' ', required = true )]
    names: Vec<String>,
}

impl super::DoDoArgs for RemoveArgs {
    fn execute(&self) -> crate::Result<()> {
        self.remove_command(None)
    }
}

impl RemoveArgs {
    fn remove_command(&self, path: Option<&str>) -> crate::Result<()> {
        let mut commands = Commands::get(path)?;

        if self
            .names
            .iter()
            .filter(|n| {
                if commands.remove(*n).is_none() {
                    println!("DoDo commands doesn't contain: {}", n.yellow().bold());
                    false
                } else {
                    println!("DoDo successfully removed command: {}", n.yellow().bold());
                    true
                }
            })
            .count()
            == 0
        {
            return Ok(());
        }
        commands.set(path)
    }
}

#[cfg(test)]
mod tests {
    use fake::{
        faker::{lorem::en::Word, name::en::Name},
        Fake,
    };
    use rand::Rng;

    use super::*;

    #[test]
    fn should_remove_all_commands_at_once() {
        let test_file = "dodo_test_remove_command.json";

        let (_, t_name) = create_test_commands(test_file);

        // remove prep
        let rm_arg = RemoveArgs {
            names: t_name.clone(),
        };
        rm_arg
            .remove_command(Some(test_file))
            .expect("Failed to remove commands");

        let rm_op_cmd =
            Commands::get(Some(test_file)).expect("Failed to get updated command file.");

        for rm_key in t_name {
            assert!(
                !rm_op_cmd.contains_key(&rm_key),
                "Expected update command to not contain key: {}",
                rm_key
            );
        }

        std::fs::remove_file(test_file).unwrap_or_else(|e| {
            panic!(
                "Expected to cleanup testfile: {}, but got: {}",
                test_file, e
            )
        });
    }

    fn create_test_commands(test_file: &str) -> (Commands, Vec<String>) {
        // create commands
        let mut test_commands =
            Commands::get(Some(test_file)).expect("Failed getting remove command json file");
        let mut t_names = Vec::<String>::new();
        for idx in 0..20 {
            let new_name = Word().fake::<String>() + &idx.to_string();
            test_commands.insert(
                new_name.clone(),
                format!("Export name=\"{}\"", Name().fake::<String>()),
            );
            if idx == 0 || rand::thread_rng().gen::<f32>() > 0.5 {
                t_names.push(new_name);
            }
        }
        test_commands
            .set(Some(test_file))
            .expect("Failed setting prep date to json file");

        (test_commands, t_names)
    }
}
