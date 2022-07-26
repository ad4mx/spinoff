use std::io::{stderr, stdout, Write};
/// Simplified type for a stream.
/// A stream that writes to stdout.
/// By default, `spinoff` uses Streams::Stdout.
/// You can use `Streams::Custom(stream)` to specify a custom stream.
#[derive(Default, Copy, Clone)]
pub enum Streams {
    #[default]
    Stdout,
    Stderr,
}

impl Streams {
    // Returns the stream to use.
    pub fn get_stream(self) -> Box<dyn Write + Send + Sync> {
        match self {
            Streams::Stdout => Box::new(stdout()),
            Streams::Stderr => Box::new(stderr()),
        }
    }
    pub fn write_fmt(self, fmt: impl std::fmt::Display) -> Result<(), std::io::Error> {
        write!(self.get_stream(), "{}", fmt)
    }
}
