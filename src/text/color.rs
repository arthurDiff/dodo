pub struct Rgb(pub u8, pub u8, pub u8);

/// ANSI escape sequence
/// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
#[allow(dead_code)]
pub trait Color {
    fn color_rgb(&self, foreground: Option<Rgb>, background: Option<Rgb>) -> String;
    fn color(&self, foreground: Option<u8>, background: Option<u8>) -> String;
    fn red(&self) -> String {
        self.color(Some(91), None)
    }
    fn yellow(&self) -> String {
        self.color(Some(93), None)
    }
    fn blue(&self) -> String {
        self.color(Some(94), None)
    }
    fn green(&self) -> String {
        self.color(Some(92), None)
    }
    fn purple(&self) -> String {
        self.color(Some(95), None)
    }
}

impl Color for str {
    /// 8 bit rgb
    fn color_rgb(&self, foreground: Option<Rgb>, background: Option<Rgb>) -> String {
        if foreground.is_none() && background.is_none() {
            return self.to_string();
        }
        let (fg_color, bg_color) = (
            foreground.map(|fg| format!("38;2;{};{};{}", fg.0, fg.1, fg.2)),
            background.map(|bg| format!("48;2;{};{};{}", bg.0, bg.1, bg.2)),
        );

        if fg_color.is_none() || bg_color.is_none() {
            return format!(
                "\x1b[{}m{}\x1b[0m",
                fg_color.unwrap_or_else(|| bg_color.unwrap()),
                self
            );
        }

        format!(
            "\x1b[{};{}m{}\x1b[0m",
            fg_color.unwrap(),
            bg_color.unwrap(),
            self
        )
    }

    /// 4 bit ANSI code
    fn color(&self, foreground: Option<u8>, background: Option<u8>) -> String {
        if foreground.is_none_or(|b| !(30..=37).contains(&b) && !(90..97).contains(&b))
            && background.is_none_or(|b| !(40..=47).contains(&b) && !(100..107).contains(&b))
        {
            return self.to_string();
        }

        if foreground.is_none() || background.is_none() {
            return format!(
                "\x1b[{}m{}\x1b[0m",
                foreground.unwrap_or_else(|| background.unwrap()),
                self
            );
        }

        format!(
            "\x1b[{};{}m{}\x1b[0m",
            foreground.unwrap(),
            background.unwrap(),
            self
        )
    }
}

impl Color for String {
    fn color_rgb(&self, foreground: Option<Rgb>, background: Option<Rgb>) -> String {
        str::color_rgb(self, foreground, background)
    }

    fn color(&self, foreground: Option<u8>, background: Option<u8>) -> String {
        str::color(self, foreground, background)
    }
}
