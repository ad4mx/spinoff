use spinoff::{Spinner, Spinners, Color};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new(Spinners::Dots, "Loading...", Color::Blue);
    sleep(Duration::from_millis(8000));
    sp.success("Done!");
}
