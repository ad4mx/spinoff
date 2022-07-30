use spinoff::{Spinner, Spinners, Streams};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new_with_stream(Spinners::Aesthetic, "Loading in stderr...", None, Streams::Stderr);
    sleep(Duration::from_millis(8000));
    sp.success("Done!");
}
