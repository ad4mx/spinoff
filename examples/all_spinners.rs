use spinoff::Spinners;
use std::thread::sleep;
use std::time::Duration;
use strum::IntoEnumIterator;
fn main() {
    for spinner in Spinners::iter() {
        let spin = spinoff::new(spinner, "Spinning...", None);
        sleep(Duration::from_secs(1));
        spin.clear();
    }
    println!("Done!");
}
