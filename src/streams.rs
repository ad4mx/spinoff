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
    #[must_use = "Stream must be retrieved"]
    pub fn get_stream(self) -> Box<dyn Write + Send + Sync> {
        match self {
            Self::Stdout => Box::new(stdout()),
            Self::Stderr => Box::new(stderr()),
        }
    }
    // Clever implementation that allows us to automatically get the stream when `write!` is called.
    pub fn write_fmt<T>(self, fmt: T)
    where
        T: std::fmt::Display,
    {
        write!(self.get_stream(), "{}", fmt).expect("error: failed to write to stream");
    }
}
