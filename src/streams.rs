use std::io::{stderr, stdout, Write};
/// Simplified type for a stream.
/// By default, `spinoff` uses `Streams::Stdout`.
#[derive(Default, Copy, Clone, Debug)]
pub enum Streams {
    #[default]
    Stdout,
    Stderr,
}

impl Streams {
    // Returns the stream to use.
    #[must_use] pub fn get_stream(self) -> Box<dyn Write + Send + Sync> {
        match self {
            Streams::Stdout => Box::new(stdout()),
            Streams::Stderr => Box::new(stderr()),
        }
    }

    pub fn write_fmt<T>(self, fmt: T)
    where
        T: std::fmt::Display,
    {
        write!(self.get_stream(), "{}", fmt).expect("error: failed to write to stream");
    }
}
