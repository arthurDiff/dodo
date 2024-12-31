use cli::DoDo;

pub use crate::error::Error;
pub use crate::result::Result;

mod cli;
mod data;
mod error;
mod result;
mod shellinfo;
mod text;

/// Features for dodo
///   run a command (or multiple concurrently)
///   settings for concurrent execution (maybe parellel) or others
///   dodo dance
///
/// Need
///   sqlite or plain text storage
///   thread pool
fn main() -> crate::Result<()> {
    DoDo::run().execute()
}
