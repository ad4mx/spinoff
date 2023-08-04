#![allow(unused_imports)]
use spinoff::{spinners, Spinner, Streams};
use std::{thread::sleep, time::Duration};

#[cfg(feature = "aesthetic")]
fn main() {
    let mut sp = Spinner::new_with_stream(spinners::Aesthetic, "Loading in stderr...", None, Streams::Stderr);
    sleep(Duration::from_millis(8000));
    sp.success("Done!");
}

#[cfg(not(feature = "aesthetic"))]
fn main() {
    println!("This example requires the 'aesthetic' feature to be enabled.");
}
