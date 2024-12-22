use clap::Args;

#[derive(Debug, Args)]
pub struct SaveArgs {
    /// name of the command
    #[arg(short, long)]
    name: String,
    /// command to be mapped to the name
    #[arg(short, long)]
    command: String,
}

impl super::DoDoArgs for SaveArgs {
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}
