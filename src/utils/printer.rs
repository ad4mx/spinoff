use yansi::Paint;

use crate::StringLiteral;

pub fn delete_last_line(msg: StringLiteral) {
    print!("\r");
    for _ in 0..(msg.len() + 30) {
        print!(" ");
    }
    print!("\r");
}

/// Accepts a color option and spinner, returns a Paint<String> object (e.g. `Paint::green(spinner)`)
pub fn init_color(color: Option<&str>, spinner: String) -> Paint<String> {
    match color {
        Some("blue") => Paint::blue(spinner),
        Some("green") => Paint::green(spinner),
        Some("red") => Paint::red(spinner),
        Some("yellow") => Paint::yellow(spinner),
        Some("cyan") => Paint::cyan(spinner),
        Some("white") => Paint::new(spinner),
        None => Paint::new(spinner),
        _ => panic!("invalid color: expected one of the following: blue, green, red, yellow, cyan, white or None"),
    }
}
