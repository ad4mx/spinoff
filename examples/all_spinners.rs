use spinoff::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};
use strum::IntoEnumIterator;

fn main() {
    for spinner in Spinners::iter() {
        let spin = Spinner::new(spinner, format!("{}", spinner), None);
        sleep(Duration::from_secs(2));
        spin.clear();
    }
}
