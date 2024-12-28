use clap::Args;

#[derive(Debug, Args)]
pub struct RunArgs {}

impl super::DoDoArgs for RunArgs {
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}
