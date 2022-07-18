//! ## spinoff
//!
//! `spinoff` is a simple library for displaying spinners in the terminal.
//!
//! ### Usage
//!
//! ```
//! # use spinoff::*;
//! # use std::thread::sleep;
//! # use std::time::Duration;
//! #
//! let sp = Spinner::new(Spinners::Dots, "Loading...", None);
//! sleep(Duration::from_millis(800));
//! sp.success("Success!");
//! ```
//!
//! ### Spinners
//!
//! There are over 80+ spinners available in the [`Spinners`] enum.
//!
//! ### Colors
//!
//! You can also color your spinners without any hassle. Simply pass a color to the `color` option.
//! There are 6 colors available: blue, green, red, yellow, cyan, white.
//! Don't want any of that? Simply pass `None` to the `color` option.

use std::borrow::Cow;
use std::io::{stdout, Write};
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::{self, JoinHandle};

/// Using this type for better readability.
type StringLiteral = &'static str;

mod printer;
mod spinner_data;
mod spinner_enum;

use printer::{delete_last_line, init_color};
use spinner_data::SPINNER_FRAMES;
pub use spinner_enum::Spinners;

/// Terminal spinner.
#[derive(Debug)]
pub struct Spinner {
    thread_handle: Option<JoinHandle<()>>,
    /// This struct has an `Arc<AtomicBool>` field, which is later used in the `stop` type methods to stop the thread printing the spinner.
    still_spinning: Arc<AtomicBool>,
    msg: Cow<'static, str>,
}

/// Color for spinner.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    Cyan,
    White,
}

impl Spinner {
    /// Create a new spinner.
    ///
    /// # Arguments
    ///
    /// * `spinner_type` - The spinner to use.
    /// * `msg` - The message to display.
    /// * `color` - The color of the spinner.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::*;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let sp = Spinner::new(Spinners::Dots, "Hello", Some(Color::Blue));
    /// sleep(Duration::from_millis(800));
    /// sp.clear();
    /// ```
    ///
    /// # Notes
    ///
    /// * The spinner immediately starts spinning upon creation.
    ///
    pub fn new<T>(spinner_type: Spinners, msg: T, color: Option<Color>) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        let still_spinning = Arc::new(AtomicBool::new(true));
        // Gain ownership of the message for the thread to use
        let msg = msg.into();
        // We use atomic bools to make the thread stop itself when the `spinner.stop()` method is called.
        let handle = thread::spawn({
            // Clone the atomic bool so that we can use it in the thread and return the original one later.
            let still_spinning = Arc::clone(&still_spinning);
            let msg = msg.clone();
            move || {
                let spinner_data = SPINNER_FRAMES.get(&spinner_type).unwrap();
                let stdout = stdout();
                // Iterate over all the frames of the spinner while the atomic bool is true.
                let frames = spinner_data
                    .frames
                    .iter()
                    .cycle()
                    .take_while(|_| still_spinning.load(std::sync::atomic::Ordering::Relaxed));

                let mut last_length = 0;
                for frame in frames {
                    // This is dependant on the stdout attached is a terminal that respects \r
                    let mut stdout_lock = stdout.lock();
                    let frame_str = format!(" {} {}", init_color(color, frame.to_string()), msg);
                    delete_last_line(last_length);
                    last_length = frame_str.bytes().len();
                    write!(stdout_lock, "{}", frame_str).unwrap();
                    stdout_lock.flush().unwrap();
                    drop(stdout_lock);

                    thread::sleep(std::time::Duration::from_millis(
                        spinner_data.interval as u64,
                    ));
                }
                delete_last_line(last_length);
            }
        });

        // Return a Spinner struct
        Self {
            thread_handle: Some(handle),
            still_spinning,
            msg,
        }
    }

    /// Stop the spinner.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let sp = Spinner::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop();
    /// ```
    ///
    /// # Notes
    ///
    /// * The spinner will be dropped after this method is called, the message will remain though.
    ///
    pub fn stop(mut self) {
        self.stop_spinner_thread();
        // print message
        println!("{}", self.msg);
    }

    /// Stops the spinner and prints a message on a new line.
    ///     
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let sp = Spinner::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop_with_message("Bye");
    /// ```
    ///
    pub fn stop_with_message(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        // put the message over the spinner
        println!("{}", msg);
    }

    /// Deletes the spinner and message and prints a new line with a symbol and message.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let sp = Spinner::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop_and_persist("🍕", "Pizza!");
    /// ```
    ///
    pub fn stop_and_persist(mut self, symbol: StringLiteral, msg: StringLiteral) {
        self.stop_spinner_thread();
        println!("{} {}", symbol, msg);
    }

    /// Deletes the last line of the terminal and prints a success symbol with a message.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #    
    /// let sp = Spinner::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.success("Success!");
    /// ```
    ///
    pub fn success(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        println!(
            "{} {}",
            init_color(Some(Color::Green), "✔".to_string()),
            &msg
        );
    }

    /// Deletes the last line of the terminal and prints a failure symbol with a message.
    ///     
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #   
    /// let sp = Spinner::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.fail("Failed!");
    /// ```
    ///
    pub fn fail(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        println!("{} {}", init_color(Some(Color::Red), "✖".to_string()), &msg);
    }

    /// Deletes the last line of the terminal and prints a warning symbol with a message.
    ///     
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #   
    /// let sp = Spinner::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.warn("Look out!");
    /// ```
    ///
    pub fn warn(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        println!(
            "{} {}",
            init_color(Some(Color::Yellow), "⚠".to_string()),
            &msg
        );
    }

    /// Deletes the last line of the terminal and prints a new spinner.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::*;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #   
    /// let mut sp = Spinner::new(Spinners::Dots, "Hello", None);
    ///
    /// sleep(Duration::from_millis(800));
    /// sp.update(Spinners::Dots2, "Goodbye", None);
    /// sleep(Duration::from_millis(800));
    ///
    /// sp.stop();
    /// ```
    ///
    pub fn update<T>(&mut self, spinner: Spinners, msg: T, color: Option<Color>)
    where
        T: Into<Cow<'static, str>>,
    {
        self.stop_spinner_thread();
        let _ = std::mem::replace(self, Self::new(spinner, msg, color));
    }

    /// Deletes the last line of the terminal.
    ///     
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let mut sp = Spinner::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.clear();
    /// ```
    ///
    pub fn clear(mut self) {
        self.stop_spinner_thread();
    }

    /// Stop the spinner thread and wait for it.
    fn stop_spinner_thread(&mut self) {
        // Set flag to signal thread to stop
        self.still_spinning
            .store(false, std::sync::atomic::Ordering::Relaxed);

        // Wait for the thread to actually stop
        // Also deletes the last line of the terminal after stopped
        self.thread_handle
            .take()
            .expect("Stopping the spinner thread should only happen once.")
            .join()
            .expect("Thread to join.");
    }
}
