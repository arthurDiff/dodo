use std::{
    io::Write,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use clap::Args;

use crate::{
    data::Commands,
    shellinfo,
    text::{Color, Font},
};

// 22 fps
const FRAME_DELAY: u64 = 40;
const LOADING_CHAR: [char; 14] = [
    '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃', '▁',
];

// need output log func (need to access child process for stio)
#[derive(Debug, Args)]
pub struct RunArgs {
    /// Names of space seperated commands to run
    names: Vec<String>,
    /// Run commands asynchronously (Default false)
    #[arg(short = 'a', long, default_value_t = false)]
    run_async: bool,
    /// Log output while command is running (Default false)
    #[arg(short = 'l', long, default_value_t = false)]
    log_while: bool,
    /// Log output on complete (Default false) | log_while will take precedence over log_output
    #[arg(short = 'o', long, default_value_t = false)]
    log_output: bool,
}

impl super::DoDoArgs for RunArgs {
    fn execute(&self) -> crate::Result<()> {
        if self.names.is_empty() {
            println!(
                "Requires at least one command to run!\nIf you need help try: {} or {}",
                "dodo list".yellow(),
                "dodo help".yellow()
            );
            return Ok(());
        }

        let sinfo = shellinfo::get_shell_name().unwrap_or(shellinfo::DEFAULT_SHELL);

        if self.run_async {
            self.run_commands_async(None, sinfo)
        } else {
            self.run_commands_sync(None, sinfo)
        }
    }
}

impl RunArgs {
    fn run_commands_async(
        &self,
        _path: Option<&str>,
        _sinfo: shellinfo::ShellInfo,
    ) -> crate::Result<()> {
        todo!()
    }

    fn run_commands_sync(
        &self,
        path: Option<&str>,
        sinfo: shellinfo::ShellInfo,
    ) -> crate::Result<()> {
        let cmds = Commands::get(path)?;
        for n in &self.names {
            let Some(cmd) = cmds.get(n) else {
                println!("DoDo commands doesn't contain: {}", n.yellow().bold());
                continue;
            };

            match if self.log_while {
                Self::run_command_w_log(n, cmd, sinfo)
            } else {
                Self::run_command(n, cmd, sinfo)
            } {
                Ok(output) => {
                    if self.log_output && !self.log_while {
                        println!(
                            "DoDo Command ({}) Ended with {}\n{}\n{}",
                            n.green(),
                            output.status,
                            "output:".yellow(),
                            if output.status.success() {
                                String::from_utf8_lossy(&output.stdout)
                            } else {
                                String::from_utf8_lossy(&output.stderr)
                            }
                        );
                    } else {
                        println!("DoDo Command ({}) Ended with {}", n.green(), output.status);
                    }
                }
                Err(err) => eprintln!("DoDo Command Failed With: {}", err.to_string().red()),
            }
        }
        Ok(())
    }

    fn run_command(
        name: &str,
        command: &str,
        sinfo: shellinfo::ShellInfo,
    ) -> crate::Result<std::process::Output> {
        let mut proc = Command::new(sinfo.0)
            .arg(sinfo.1)
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut idx = 0;
        let mut start_inst = Instant::now();
        while proc.try_wait().is_ok_and(|p| p.is_none()) {
            print!(
                "\rDoDo Command ({}) Running {}",
                name.green(),
                LOADING_CHAR[idx % 14]
            );
            std::io::stdout().flush()?;
            let duration_since = Instant::now().duration_since(start_inst);
            if Duration::from_millis(FRAME_DELAY) > duration_since {
                std::thread::sleep(Duration::from_millis(FRAME_DELAY) - duration_since)
            }
            start_inst = Instant::now();
            idx = idx.wrapping_add(1);
        }

        print!("\r");
        std::io::stdout().flush()?;
        Ok(proc.wait_with_output()?)
    }

    fn run_command_w_log(
        name: &str,
        command: &str,
        sinfo: shellinfo::ShellInfo,
    ) -> crate::Result<std::process::Output> {
        println!("DoDo Command ({}) Running...", name.green());
        // it will write to stdout or stderr while process is running;
        let proc = Command::new(sinfo.0).arg(sinfo.1).arg(command).spawn()?;
        Ok(proc.wait_with_output()?)
    }
}
