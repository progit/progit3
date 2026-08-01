//! Small console-output helpers. Colours are auto-disabled when stdout is not
//! a TTY or when `NO_COLOR` is set.

use std::io::IsTerminal;
use std::sync::OnceLock;

fn colour_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
    })
}

fn paint(code: &str, text: &str) -> String {
    if colour_enabled() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// A step is starting.
pub fn step(msg: &str) {
    println!("{} {}", paint("1;34", "==>"), msg);
}

/// A step finished successfully, usually naming the output file.
pub fn done(msg: &str) {
    println!("    {} {}", paint("1;32", "✓"), msg);
}

/// Informational note.
pub fn info(msg: &str) {
    println!("    {msg}");
}

/// A non-fatal warning.
pub fn warn(msg: &str) {
    eprintln!("{} {}", paint("1;33", "warning:"), msg);
}

/// A fatal error message (printed just before exit).
pub fn error(msg: &str) {
    eprintln!("{} {}", paint("1;31", "error:"), msg);
}
