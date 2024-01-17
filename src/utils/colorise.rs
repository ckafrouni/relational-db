pub trait Coloured {
    fn bold(self) -> String;
    fn faint(self) -> String;
    fn italic(self) -> String;
    fn underline(self) -> String;
    fn blink(self) -> String;
    fn reverse(self) -> String;

    fn black(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn magenta(self) -> String;
    fn cyan(self) -> String;
    fn white(self) -> String;

    fn on_black(self) -> String;
    fn on_red(self) -> String;
    fn on_green(self) -> String;
    fn on_yellow(self) -> String;
    fn on_blue(self) -> String;
    fn on_magenta(self) -> String;
    fn on_cyan(self) -> String;
    fn on_white(self) -> String;
}

impl<T: AsRef<str>> Coloured for T {
    // Style
    fn bold(self) -> String {
        format!("\x1b[1m{}\x1b[0m", self.as_ref())
    }
    fn faint(self) -> String {
        format!("\x1b[2m{}\x1b[0m", self.as_ref())
    }
    fn italic(self) -> String {
        format!("\x1b[3m{}\x1b[0m", self.as_ref())
    }
    fn underline(self) -> String {
        format!("\x1b[4m{}\x1b[0m", self.as_ref())
    }
    fn blink(self) -> String {
        format!("\x1b[5m{}\x1b[0m", self.as_ref())
    }
    fn reverse(self) -> String {
        format!("\x1b[7m{}\x1b[0m", self.as_ref())
    }

    // Foreground
    fn black(self) -> String {
        format!("\x1b[30m{}\x1b[0m", self.as_ref())
    }
    fn red(self) -> String {
        format!("\x1b[31m{}\x1b[0m", self.as_ref())
    }
    fn green(self) -> String {
        format!("\x1b[32m{}\x1b[0m", self.as_ref())
    }
    fn yellow(self) -> String {
        format!("\x1b[33m{}\x1b[0m", self.as_ref())
    }
    fn blue(self) -> String {
        format!("\x1b[34m{}\x1b[0m", self.as_ref())
    }
    fn magenta(self) -> String {
        format!("\x1b[35m{}\x1b[0m", self.as_ref())
    }
    fn cyan(self) -> String {
        format!("\x1b[36m{}\x1b[0m", self.as_ref())
    }
    fn white(self) -> String {
        format!("\x1b[37m{}\x1b[0m", self.as_ref())
    }

    // Background
    fn on_black(self) -> String {
        format!("\x1b[40m{}\x1b[0m", self.as_ref())
    }
    fn on_red(self) -> String {
        format!("\x1b[41m{}\x1b[0m", self.as_ref())
    }
    fn on_green(self) -> String {
        format!("\x1b[42m{}\x1b[0m", self.as_ref())
    }
    fn on_yellow(self) -> String {
        format!("\x1b[43m{}\x1b[0m", self.as_ref())
    }
    fn on_blue(self) -> String {
        format!("\x1b[44m{}\x1b[0m", self.as_ref())
    }
    fn on_magenta(self) -> String {
        format!("\x1b[45m{}\x1b[0m", self.as_ref())
    }
    fn on_cyan(self) -> String {
        format!("\x1b[46m{}\x1b[0m", self.as_ref())
    }
    fn on_white(self) -> String {
        format!("\x1b[47m{}\x1b[0m", self.as_ref())
    }
}
