use clap::Args;

#[derive(Debug, Args)]
pub struct ListArgs;

impl super::DoDoArgs for ListArgs {
    fn execute(&self) -> crate::Result<()> {
        let commands = crate::data::Commands::get(None)?;
        let mut l_str = "----Commands----".to_string();
        for (k, v) in commands.iter() {
            l_str += &format!("\n{k} : {v}");
        }
        l_str += "----END----";
        println!("{}", l_str);
        Ok(())
    }
}
