#![allow(unused_imports)]
use spinoff::{spinners, Color, Spinner};
use std::{thread::sleep, time::Duration};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

#[cfg(feature = "dots")]
fn main() {    
    let spinner = Arc::new(
            Mutex::new(
                Spinner::new(spinners::Dots, "Loading...", Color::Blue)
            )
    );
    let mut handles = vec![];

    for t in 0..5 {
        let spinner = Arc::clone(&spinner);
        let handle = spawn(move||{
            sleep(Duration::from_secs(t));
            let mut tspinner = spinner.lock().unwrap();
            
            tspinner.update_text(format!("In Thread {}", t));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    spinner.lock().unwrap().success("Complete");

}
