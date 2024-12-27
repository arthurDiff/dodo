use clap::Args;

#[derive(Debug, Args)]
pub struct AddArgs {
    /// name of the command
    #[arg(short, long)]
    name: String,
    /// command to be mapped to the name
    #[arg(short, long)]
    command: String,
}

impl super::DoDoArgs for AddArgs {
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}
