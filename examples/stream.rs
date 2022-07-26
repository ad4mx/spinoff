use spinoff::{Spinner, Spinners, Streams};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new_with_stream(Spinners::Dots, "Loading...", None, Streams::Stdout);
    sleep(Duration::from_millis(8000));
    sp.success("Done!");
}
