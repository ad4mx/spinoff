use spinoff::Spinners;
use std::thread::sleep;
use std::time::Duration;
use strum::IntoEnumIterator;
fn main() {
    let mut spin = spinoff::new(Spinners::Dots, "Spinning...", None);
    for spinner in Spinners::iter() {
        spin = spin.update(spinner, "Spinning...", None);
        sleep(Duration::from_secs(1));
    }
    println!("Done!");
}
