use clap::Args;

#[derive(Debug, Args)]
pub struct RemoveArgs {}

impl super::DoDoArgs for RemoveArgs {
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}
