use spinoff::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new(Spinners::Dots, "Loading...", None);
    sleep(Duration::from_millis(8000));
    sp.success("Done!");
}
