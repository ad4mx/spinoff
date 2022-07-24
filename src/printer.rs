use crate::{Color, Stream, StringLiteral};
use std::io::Write;
use yansi::Paint;

pub fn delete_last_line(clear_length: usize, stream: &mut Stream) -> Result<(), std::io::Error> {
    write!(stream, "\r")?;
    for _ in 0..clear_length {
        write!(stream, " ")?;
    }
    write!(stream, "\r")?;
    Ok(())
}

/// Accepts a color option and spinner, returns a Paint<String> object (e.g. `Paint::green(spinner)`)
pub fn init_color(color: Option<Color>, spinner: StringLiteral) -> Paint<StringLiteral> {
    match color {
        Some(Color::Blue) => Paint::blue(spinner),
        Some(Color::Green) => Paint::green(spinner),
        Some(Color::Red) => Paint::red(spinner),
        Some(Color::Yellow) => Paint::yellow(spinner),
        Some(Color::Cyan) => Paint::cyan(spinner),
        Some(Color::White) => Paint::new(spinner),
        Some(Color::Black) => Paint::black(spinner),
        Some(Color::Magenta) => Paint::magenta(spinner),
        None => Paint::new(spinner),
    }
}
