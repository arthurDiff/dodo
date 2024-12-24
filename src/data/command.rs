const COMMAND_FILE_PATH: &str = "dodo_config.json";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Commands(std::collections::HashMap<String, String>);

impl super::DoDoData for Commands {
    fn read() -> crate::Result<Self> {
        todo!()
    }

    fn write(&self) -> crate::Result<()> {
        todo!()
    }
}

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
