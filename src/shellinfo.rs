// Shell and command start option
const SHELLS: [(&str, &str); 10] = [
    ("bash", "-c"),
    ("zsh", "-c"),
    ("cmd", "/C"),
    ("powershell", "-Command"),
    ("sh", "-c"),
    ("wsl", "-e bash -c"),
    ("python", "-c"),
    ("ruby", "-e"),
    ("fish", "-c"),
    ("ksh", "-c"),
];

pub const DEFAULT_SHELL: (&str, &str) = SHELLS[2];

pub type ShellInfo = (&'static str, &'static str);

pub fn get_shell_name() -> Option<(&'static str, &'static str)> {
    let mut sys = sysinfo::System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, false);

    let mut pid = sysinfo::Pid::from_u32(std::process::id());
    while let Some(proc) = sys.process(pid) {
        let proc_name = proc.name().to_str()?;
        if let Some(s) = SHELLS
            .iter()
            .find(|(s_name, _)| proc_name.starts_with(s_name))
        {
            return Some((s.0, s.1));
        }
        pid = proc.parent()?
    }

    None
}
