use spinoff::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};
use strum::IntoEnumIterator;

fn main() {
    let mut spin = Spinner::new(Spinners::Dots, "", None);
    for spinner in Spinners::iter() {
        spin.update(spinner, format!("{}", spinner), None);
        sleep(Duration::from_secs(2));
    }
    spin.stop_with_message("Done!");
}
