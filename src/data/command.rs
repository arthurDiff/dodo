pub const DEFAULT_COMMAND_FILE_PATH: &str = "dodo_config.json";

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Commands(std::collections::HashMap<String, String>);

impl super::DoDoData for Commands {}

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
