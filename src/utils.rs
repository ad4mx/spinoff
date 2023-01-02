use crate::Streams;
use colored::{ColoredString, Colorize};

/// Color for spinner. Supports the 8 basic colors and a custom color variant.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    Cyan,
    White,
    Black,
    Magenta,
    TrueColor { r: u8, g: u8, b: u8 },
}

pub fn colorize(color: Option<Color>, frame: &str) -> ColoredString {
    match color {
        Some(Color::Blue) => frame.blue(),
        Some(Color::Green) => frame.green(),
        Some(Color::Red) => frame.red(),
        Some(Color::Yellow) => frame.yellow(),
        Some(Color::Cyan) => frame.cyan(),
        Some(Color::White) => frame.white(),
        Some(Color::Black) => frame.black(),
        Some(Color::Magenta) => frame.magenta(),
        Some(Color::TrueColor { r, g, b }) => frame.truecolor(r, g, b),
        None => frame.normal()
    }
}

/// Internal function for deleting the last line in a terminal.
/// This is used to clear the spinner.
pub fn delete_last_line(clear_length: usize, stream: Streams) {
    write!(stream, "\r");
    for _ in 0..clear_length {
        write!(stream, " ");
    }
    write!(stream, "\r");
}


