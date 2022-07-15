use spinoff::Spinners;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let sp = spinoff::new(Spinners::Dots, "Loading...", None);
    sleep(Duration::from_millis(800));
    sp.success("Success!");
}
