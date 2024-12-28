use clap::Args;

#[derive(Debug, Args)]
pub struct ListArgs;

impl super::DoDoArgs for ListArgs {
    fn execute(&self) -> crate::Result<()> {
        let commands = crate::data::Commands::get(None)?;
        let mut l_str = String::new();
        for (k, v) in commands.iter() {
            l_str += &format!("\n{k} : {v}");
        }
        println!("----Commands----\n{}\n----END----", l_str);
        Ok(())
    }
}
