use std::{io::Write, path::PathBuf};

// Initial Impl using Json
mod command;

pub use command::*;

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

pub fn get_relative_to_bin(path: &str) -> crate::Result<String> {
    Ok(get_relative_to_bin_as_pathbuf(path)?.display().to_string())
}

pub fn get_relative_to_bin_as_pathbuf(path: &str) -> crate::Result<PathBuf> {
    let mut exe_path = std::env::current_exe()?
        .parent()
        .expect("Failed to get parent dir of binary")
        .to_path_buf();
    exe_path.push(path);
    Ok(std::path::absolute(&exe_path)?)
}
