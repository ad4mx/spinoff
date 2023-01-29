#![allow(unused_imports)]
use spinoff::{spinners, Color, Spinner};
use std::{thread::sleep, time::Duration};

#[cfg(feature = "arc")]
fn main() {
    let sp = Spinner::new(spinners::Arc, "Loading...", Color::Blue);
    sleep(Duration::from_secs(5));
    sp.stop_and_persist("🍕", "Pizza!");
}

#[cfg(not(feature = "arc"))]
fn main() {
    println!("This example requires the 'arc' feature to be enabled.");
}
