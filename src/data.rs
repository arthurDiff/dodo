use std::io::Write;

// Initial Impl using Json
mod command;
mod config;

pub use command::Commands;
pub use config::Config;

#[allow(dead_code)]
fn upsert_json<'a, T>(data_path: std::path::PathBuf, data: &T) -> crate::Result<()>
where
    T: serde::Serialize + serde::Deserialize<'a>,
{
    std::fs::File::create(data_path)
        .map_err(crate::Error::IOError)?
        .write_all(
            serde_json::to_string(data)
                .map_err(crate::Error::SerializationError)?
                .as_bytes(),
        )
        .map_err(crate::Error::IOError)
}
