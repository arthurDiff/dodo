use super::DoDoData;

const DEFAULT_CONFIG_FILE_PATH: &str = "dodo_config.json";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub thread_count: u8,
}

impl Config {
    /// If None will use default path
    pub fn get(path: Option<&str>) -> crate::Result<Self> {
        Self::read(path.unwrap_or(DEFAULT_CONFIG_FILE_PATH))
    }

    /// If None will use default path
    pub fn set(&self, path: Option<&str>) -> crate::Result<()> {
        self.write(path.unwrap_or(DEFAULT_CONFIG_FILE_PATH))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { thread_count: 4 }
    }
}

impl DoDoData for Config {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write_config_file_correctly() {
        let test_file_path = "test_config.json";
        let mut default_config = Config::default();

        let test_config = Config::get(Some(test_file_path)).unwrap();

        assert_eq!(test_config.thread_count, default_config.thread_count);

        default_config.thread_count = 8;

        let update_req = default_config.set(Some(test_file_path));
        assert!(update_req.is_ok());

        let updated_config = Config::get(Some(test_file_path)).unwrap();
        assert_eq!(updated_config.thread_count, 8);

        // cleanup
        let _ = std::fs::remove_file(test_file_path);
    }
}
