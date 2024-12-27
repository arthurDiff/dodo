use clap::Args;

#[derive(Debug, Args)]
pub struct AboutArgs;

impl super::DoDoArgs for AboutArgs {
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}
