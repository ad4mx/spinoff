use spinoff::{Spinner, Spinners, Streams};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new_with_stream(Spinners::Dots, "Loading in stderr...", None, Streams::Stderr);
    sleep(Duration::from_millis(800));
    sp.success("Done!");
}
