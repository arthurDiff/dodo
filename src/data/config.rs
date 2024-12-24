const CONFIG_FILE_PATH: &str = "dodo_config.json";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub thread_count: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self { thread_count: 4 }
    }
}

impl super::DoDoData for Config {
    fn read() -> crate::Result<Self> {
        match std::fs::read_to_string(CONFIG_FILE_PATH) {
            Ok(config_str) => serde_json::from_str::<Config>(&config_str)
                .map_err(crate::Error::SerializationError),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    let new_config = Self::default();
                    super::upsert_json(CONFIG_FILE_PATH.into(), &new_config)?;
                    return Ok(new_config);
                }
                Err(crate::Error::IOError(err))
            }
        }
    }

    fn write(&self) -> crate::Result<()> {
        super::upsert_json(CONFIG_FILE_PATH.into(), self)?;
        Ok(())
    }
}
