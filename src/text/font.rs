/// ANSI escape sequence
/// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
pub trait Font {
    fn bold(&self) -> String;
    fn faint(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn crossed_out(&self) -> String;
}

impl Font for str {
    fn bold(&self) -> String {
        format!("\x1b[1m{}\x1b[0m", self)
    }

    fn faint(&self) -> String {
        format!("\x1b[2m{}\x1b[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1b[3m{}\x1b[0m", self)
    }

    fn underline(&self) -> String {
        format!("\x1b[4m{}\x1b[0m", self)
    }

    fn crossed_out(&self) -> String {
        format!("\x1b[9m{}\x1b[0m", self)
    }
}

impl Font for String {
    fn bold(&self) -> String {
        str::bold(self)
    }

    fn faint(&self) -> String {
        str::faint(self)
    }

    fn italic(&self) -> String {
        str::italic(self)
    }

    fn underline(&self) -> String {
        str::underline(self)
    }

    fn crossed_out(&self) -> String {
        str::crossed_out(self)
    }
}
