#![allow(unused_imports)]
use spinoff::{spinners, Color, Spinner};
use std::{thread::sleep, time::Duration};

/// This example requires the 'dots' feature to be enabled (the 'dots' feature
/// is enabled by default).
#[cfg(feature = "dots")]
fn main() {
    let sp = Spinner::new(spinners::Dots, "Loading...", Color::Blue);
    sleep(Duration::from_millis(8000));
    sp.success("Done!");
}

#[cfg(not(feature = "dots"))]
fn main() {
    println!("This example requires the 'dots' feature to be enabled.");
}
