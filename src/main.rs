use cli::DoDo;

pub use crate::error::Error;
pub use crate::result::Result;

mod cli;
mod data;
mod error;
mod result;

/// Features for dodo
///   run a command (or multiple concurrently)
///   save new command
///     - from typed string
///     - from file
///     - from other command exec files like makefile (maybe)
///   delete command
///   list command
///   settings for concurrent execution (maybe parellel) or others
///   dodo dance
///
/// Need
///   sqlite or plain text storage
///   config parser
///   thread pool
///   
fn main() {
    let cli = DoDo::run();

    println!("this is running---{:?}", cli);
}
