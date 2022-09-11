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

// Internal macro for coloring text with a supplied Color enum variant.
// Why not just match? Having matching automated helps with scaling and readability.
// TODO: This macro requires a call with all the variants listed. We could rewrite this to not require that.
macro_rules! color {
    (
        $($variant:ident: $paint_method:ident,)*
    ) => {
        pub fn colorize(color: Option<Color>, spinner: &str) -> ColoredString {
            match color {
                $(
                    Some(Color::$variant) => spinner.$paint_method(),
                )*
                Some(Color::TrueColor {r, g, b}) => spinner.truecolor(r, g, b),
                None => spinner.normal()
            }
        }
    };
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

color!(
    Red: red,
    Green: green,
    Yellow: yellow,
    Blue: blue,
    Magenta: magenta,
    Cyan: cyan,
    White: white,
    Black: black,
);
