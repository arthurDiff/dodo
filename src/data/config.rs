pub const DEFAULT_CONFIG_FILE_PATH: &str = "dodo_config.json";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub thread_count: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self { thread_count: 4 }
    }
}

impl super::DoDoData for Config {}
