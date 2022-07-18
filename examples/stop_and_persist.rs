use spinoff::{Spinner, Spinners, Color};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new(Spinners::Dots, "Loading...", Some(Color::Blue));
    sleep(Duration::from_secs(5));
    sp.stop_and_persist("🍕", "Pizza!");
}
