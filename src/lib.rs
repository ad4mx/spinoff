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
//! There are 9 colors available: blue, green, red, yellow, cyan, white, magenta, black and a custom variant.
//! Don't want any of that? Simply pass `None` to the `color` option.
#![allow(clippy::nursery)]
use std::borrow::Cow;
use std::io::Write;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::{self, JoinHandle};
use colored::Colorize;

mod printer;
mod spinner_data;
mod spinner_enum;
mod streams;

use printer::{delete_last_line, colorize};
use spinner_data::SPINNER_FRAMES;
pub use spinner_enum::Spinners;
pub use streams::Streams;
pub use printer::Color;

/// Terminal spinner.
#[derive(Debug)]
pub struct Spinner {
    thread_handle: Option<JoinHandle<()>>,
    /// This struct has an `Arc<AtomicBool>` field, which is later used in the `stop` type methods to stop the thread printing the spinner.
    still_spinning: Arc<AtomicBool>,
    spinner_type: Spinners,
    msg: Cow<'static, str>,
    stream: Streams,
    color: Option<Color>,
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
    /// let sp = Spinner::new(Spinners::Dots, "Hello World!", Color::Blue);
    /// sleep(Duration::from_millis(800));
    /// sp.stop();
    /// ```
    ///
    /// # Notes
    ///
    /// * The spinner immediately starts spinning upon creation.
    /// * This function outputs to the `stdout` stream. If you want to use a different stream, use the [`Spinner::new_with_stream`] function.
    pub fn new<T, U>(spinner_type: Spinners, msg: T, color: U) -> Self
    where
        T: Into<Cow<'static, str>>,
        U: Into<Option<Color>>,
    {
        Self::new_with_stream(spinner_type, msg, color, Streams::default())
    }
    /// Create a new spinner outputting to a specific stream.
    ///
    /// # Arguments
    ///
    /// * `spinner_type` - The spinner to use.
    /// * `msg` - The message to display.
    /// * `color` - The color of the spinner.
    /// * `stream` - The stream to output to.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::*;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let sp = Spinner::new_with_stream(Spinners::Dots, "I'm outputting to stderr!", Color::Yellow, Streams::Stderr);
    /// sleep(Duration::from_millis(800));
    /// sp.clear();
    /// ```
    ///
    /// # Notes
    ///
    /// * The spinner immediately starts spinning upon creation.
    ///
    pub fn new_with_stream<T, U>(spinner_type: Spinners, msg: T, color: U, stream: Streams) -> Self
    where
        T: Into<Cow<'static, str>>,
        U: Into<Option<Color>>,
    {
        let still_spinning = Arc::new(AtomicBool::new(true));
        // Gain ownership of the message and color for the thread to use
        let msg = msg.into();
        let color = color.into();
        // We use atomic bools to make the thread stop itself when the `spinner.stop()` method is called.
        let handle = thread::spawn({
            // Clone the atomic bool so that we can use it in the thread and return the original one later.
            let still_spinning = Arc::clone(&still_spinning);
            let msg = msg.clone();
            move || {
                let spinner_data = SPINNER_FRAMES.get(&spinner_type).expect("error: invalid spinner type");
                // Iterate over all the frames of the spinner while the atomic bool is true.
                let frames = spinner_data
                    .frames
                    .iter()
                    .cycle()
                    .take_while(|_| still_spinning.load(std::sync::atomic::Ordering::Relaxed));
                // Dynamically delete the last line of the terminal depending on the length of the message + spinner.
                let mut last_length = 0;
                for frame in frames {
                    let frame_str = format!(" {} {}", colorize(color, frame), msg);
                    // Get us back to the start of the line.
                    delete_last_line(last_length, stream);
                    last_length = frame_str.bytes().len();
                    write!(stream, "{}", frame_str);
                    stream.get_stream().flush().expect("error: failed to flush stream");

                    thread::sleep(std::time::Duration::from_millis(
                        u64::from(spinner_data.interval),
                    ));
                }
                delete_last_line(last_length, stream);
            }
        });

        // Return a Spinner struct
        Self {
            thread_handle: Some(handle),
            still_spinning,
            spinner_type,
            msg,
            stream,
            color,
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
    /// let sp = Spinner::new(Spinners::Dots9, "Spinning...", None);
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
        writeln!(self.stream, "{}", self.msg);
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
    /// let sp = Spinner::new(Spinners::Dots2, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop_with_message("Bye");
    /// ```
    ///
    pub fn stop_with_message(mut self, msg: &str) {
        self.stop_spinner_thread();
        // put the message over the spinner
        writeln!(self.stream, "{}", msg);
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
    /// let sp = Spinner::new(Spinners::Mindblown, "Guess what's coming...", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop_and_persist("🍕", "Pizza!");
    /// ```
    ///
    pub fn stop_and_persist(mut self, symbol: &str, msg: &str) {
        self.stop_spinner_thread();
        writeln!(self.stream, "{} {}", symbol, msg);
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
    /// let sp = Spinner::new(Spinners::Aesthetic, "Trying to load information...", None);
    /// sleep(Duration::from_millis(800));
    /// sp.success("Success!");
    /// ```
    ///
    pub fn success(mut self, msg: &str) {
        self.stop_spinner_thread();
        writeln!(self.stream, "{} {}", colorize(Some(Color::Green), "✓").bold(), msg);
    }

    /// Deletes the last line of the terminal and prints a failure symbol with a message to stderr.
    ///     
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner, Color};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #   
    /// let sp = Spinner::new(Spinners::BouncingBar, "Executing code...", Color::Green);
    /// sleep(Duration::from_millis(800));
    /// sp.fail("Code failed to compile!");
    /// ```
    ///
    pub fn fail(mut self, msg: &str) {
        self.stop_spinner_thread();
        writeln!(self.stream, "{} {}", colorize(Some(Color::Red), "✗").bold(), msg);
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
    /// let sp = Spinner::new(Spinners::Material, "Measuring network speed...", None);
    /// sleep(Duration::from_millis(800));
    /// sp.warn("You might want to check your internet connection...");
    /// ```
    ///
    pub fn warn(mut self, msg: &str) {
        self.stop_spinner_thread();
        writeln!(self.stream, "{} {}", colorize(Some(Color::Yellow), "⚠").bold(), msg);
    }
    /// Deletes the last line of the terminal and prints an info symbol with a message.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::{Spinners, Spinner};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    ///  
    /// let sp = Spinner::new(Spinners::Dots9, "Loading info message...", None);
    /// sleep(Duration::from_millis(800));
    /// sp.info("This is an info message!");    
    /// ```
    ///
    pub fn info(mut self, msg: &str) {
        self.stop_spinner_thread();
        writeln!(self.stream, "{} {}", colorize(Some(Color::Blue), "ℹ").bold(), msg);
    }

    /// Updates the spinner.
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
    /// sp.update(Spinners::Dots2, "World", None);
    /// sleep(Duration::from_millis(800));
    ///
    /// sp.stop();
    /// ```
    ///
    pub fn update<T, U>(&mut self, spinner: Spinners, msg: T, color: U)
    where
        T: Into<Cow<'static, str>>,
        U: Into<Option<Color>>,
    {
        self.stop_spinner_thread();
        let _replaced = std::mem::replace(self, Self::new_with_stream(spinner, msg, color, self.stream));
    }

    /// Update the spinner text.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::*;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let mut sp = Spinner::new(Spinners::Arc, "Loading...", Color::Magenta);
    /// sleep(Duration::from_millis(800));
    /// sp.update_text("Not quite finished...");
    /// sleep(Duration::from_millis(800));
    /// sp.update_text("Almost done...");
    /// sleep(Duration::from_millis(800));
    /// sp.success("Done!");
    ///
    pub fn update_text<T>(&mut self, msg: T)
    where
        T: Into<Cow<'static, str>>,
    {
        self.stop_spinner_thread();
        let _replaced = std::mem::replace(self, Self::new_with_stream(self.spinner_type, msg, self.color, self.stream));
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
    /// let sp = Spinner::new(Spinners::Grenade, "Clearing...", None);
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
