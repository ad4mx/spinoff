use crate::{Color, StringLiteral};
use std::io::{stdout, Write};
use yansi::Paint;

pub fn delete_last_line(clear_length: usize) {
    let mut stdout_lock = stdout().lock();
    write!(stdout_lock, "\r").unwrap();
    for _ in 0..clear_length {
        write!(stdout_lock, " ").unwrap();
    }
    write!(stdout_lock, "\r").unwrap();

    drop(stdout_lock);
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
