use super::DoDoData;

const DEFAULT_COMMAND_FILE_PATH: &str = "dodo_commands.json";

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Commands(std::collections::HashMap<String, String>);

impl Commands {
    /// If None will use default path
    pub fn get(path: Option<&str>) -> crate::Result<Self> {
        Self::read(path.unwrap_or(DEFAULT_COMMAND_FILE_PATH))
    }

    /// If None will use default path
    pub fn set(&self, path: Option<&str>) -> crate::Result<()> {
        self.write(path.unwrap_or(DEFAULT_COMMAND_FILE_PATH))
    }
}

impl DoDoData for Commands {}

impl std::ops::Deref for Commands {
    type Target = std::collections::HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Commands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write_command_file_correctly() {
        let test_file_path = "test_command.json";

        let mut test_commands = Commands::get(Some(test_file_path)).unwrap();

        let test_vals = [
            ("hi", "world"),
            ("hello", "world too"),
            ("bye", "world bye"),
        ];

        for (k, v) in test_vals.iter() {
            test_commands.insert(k.to_string(), v.to_string());
        }

        // update commands file
        test_commands.set(Some(test_file_path)).unwrap();

        let updated_commands = Commands::get(Some(test_file_path)).unwrap();
        assert_eq!(updated_commands.len(), 3);

        for (k, v) in test_vals {
            assert_eq!(updated_commands.get(k).unwrap(), v);
        }

        // cleanup
        std::fs::remove_file(test_file_path).unwrap();
    }
}
