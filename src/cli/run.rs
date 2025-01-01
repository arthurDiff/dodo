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

// 30 fps
const FRAME_DELAY: u64 = 33;
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
        path: Option<&str>,
        sinfo: shellinfo::ShellInfo,
    ) -> crate::Result<()> {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let cmds = Commands::get(path)?;
        self.names.par_iter().for_each(|n| {
            let Some(cmd) = cmds.get(n) else {
                println!("DoDo commands doesn't contain: {}", n.yellow().bold());
                return;
            };
            // std::iter::repeat("\n").take(idx).collect::<String>();
            self.run_command(n, cmd, sinfo);
        });
        Ok(())
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
            self.run_command(n, cmd, sinfo);
        }
        Ok(())
    }

    fn run_command(&self, n: &str, cmd: &str, sinfo: shellinfo::ShellInfo) {
        match if self.log_while {
            Self::run_command_inherited(n, cmd, sinfo)
        } else {
            Self::run_command_piped(n, cmd, sinfo)
        } {
            Ok(output) => {
                if self.log_output && !self.log_while {
                    println!(
                        "DoDo Command ({}) Ended with {}\n{}\n{}",
                        n.green(),
                        output.status,
                        "Output".yellow().underline(),
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

    /// Doesn't log while command is running
    fn run_command_piped(
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
                "DoDo Command ({}) Running {}\r",
                name.green(),
                // just gonna slow down by deviding instead delaying framerate
                LOADING_CHAR[idx % 28 / 2]
            );
            std::io::stdout().flush()?;
            let duration_since = Instant::now().duration_since(start_inst);
            if Duration::from_millis(FRAME_DELAY) > duration_since {
                std::thread::sleep(Duration::from_millis(FRAME_DELAY) - duration_since)
            }
            start_inst = Instant::now();
            idx = idx.wrapping_add(1)
        }

        Ok(proc.wait_with_output()?)
    }

    /// Logs to stdout while command is running
    fn run_command_inherited(
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
