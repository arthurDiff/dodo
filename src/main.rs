use cli::DoDo;

pub use crate::error::Error;
pub use crate::result::Result;

mod cli;
mod data;
mod error;
mod result;
mod shellinfo;
mod text;

fn main() -> crate::Result<()> {
    DoDo::run().execute()
}
