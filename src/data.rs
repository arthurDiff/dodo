use std::io::Write;

// Initial Impl using Json
mod command;
mod config;

pub use command::*;
pub use config::*;

pub trait DoDoData {
    fn read(path: &str) -> crate::Result<Self>
    where
        Self: std::marker::Sized + Default + serde::Serialize + serde::de::DeserializeOwned,
    {
        read_json(path)
    }

    fn write(&self, path: &str) -> crate::Result<()>
    where
        Self: std::marker::Sized + Default + serde::Serialize + serde::de::DeserializeOwned,
    {
        write_json(path, self)
    }
}

fn write_json<T>(file_path: &str, data: &T) -> crate::Result<()>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    Ok(std::fs::File::create(file_path)?.write_all(&serde_json::to_vec(data)?)?)
}

fn read_json<T>(file_path: &str) -> crate::Result<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    match std::fs::read(file_path) {
        Ok(file_b) => Ok(serde_json::from_slice::<T>(&file_b)?),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                let default = T::default();
                write_json(file_path, &default)?;
                return Ok(default);
            }
            Err(crate::Error::IOError(err))
        }
    }
}
