use spinoff::{Spinner, Spinners, Streams};
use std::{thread::sleep, time::Duration};

fn main() {
    // You can also define your own stream, and input it with `Streams::Custom(stream)`
    let sp = Spinner::new_with_stream(Spinners::Dots, "Loading...", None, Streams::Stderr);
    sleep(Duration::from_millis(8000));
    sp.success("Done!");
}
