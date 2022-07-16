use spinoff::Spinners;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sp = spinoff::new(Spinners::Dots, "Loading...", None);
    sleep(Duration::from_secs(5));
    sp.stop_and_persist("🍕", "Pizza!");
}
