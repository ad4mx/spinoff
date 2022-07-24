use std::io::{stderr, stdout, Write};
/// Simplified type for a stream.
pub type Stream = Box<dyn Write + Send + Sync>;
/// A stream that writes to stdout.
/// By default, `spinoff` uses Streams::Stdout.
/// You can use `Streams::Custom(stream)` to specify a custom stream.
#[derive(Default)]
pub enum Streams {
    #[default]
    Stdout,
    Stderr,
    Custom(Stream),
}

impl Streams {
    // Returns the stream to use.
    pub fn get_stream(self) -> Stream {
        match self {
            Streams::Stdout => Box::new(stdout()),
            Streams::Stderr => Box::new(stderr()),
            Streams::Custom(stream) => stream,
        }
    }
}
