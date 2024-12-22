use clap::Args;

#[derive(Debug, Args)]
pub struct SaveArgs {}

impl super::DoDoArgs for SaveArgs {
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}
