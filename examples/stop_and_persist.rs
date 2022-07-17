use spinoff::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new(Spinners::Dots, "Loading...", None);
    sleep(Duration::from_secs(5));
    sp.stop_and_persist("🍕", "Pizza!");
}
