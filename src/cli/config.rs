use clap::Args;

#[derive(Debug, Args)]
pub struct ConfigArgs {}

impl super::DoDoArgs for ConfigArgs{
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}