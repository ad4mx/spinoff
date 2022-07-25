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
use std::io::Write;
use std::sync::Mutex;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::{self, JoinHandle};

/// Using this type for better readability.
type StringLiteral = &'static str;
mod printer;
mod spinner_data;
mod spinner_enum;
mod streams;

use printer::{delete_last_line, init_color};
use spinner_data::SPINNER_FRAMES;
pub use spinner_enum::Spinners;
use streams::Stream;
pub use streams::Streams;

/// Terminal spinner.
pub struct Spinner {
    thread_handle: Option<JoinHandle<()>>,
    /// This struct has an `Arc<AtomicBool>` field, which is later used in the `stop` type methods to stop the thread printing the spinner.
    still_spinning: Arc<AtomicBool>,
    msg: Cow<'static, str>,
    stream: Arc<Mutex<Stream>>,
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
    Black,
    Magenta,
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
    /// sp.clear();
    /// ```
    ///
    /// # Notes
    ///
    /// * The spinner immediately starts spinning upon creation.
    ///
    pub fn new<T, U>(spinner_type: Spinners, msg: T, color: U) -> Self
    where
        T: Into<Cow<'static, str>>,
        U: Into<Option<Color>>,
    {
        Self::new_with_stream(spinner_type, msg, color, Streams::default())
    }
    /// Create a new spinner with a defined output stream.
    ///
    /// # Arguments
    ///
    /// * `spinner_type` - The spinner to use.
    /// * `msg` - The message to display.
    /// * `color` - The color of the spinner.
    /// * `stream` - The stream to use.
    ///
    /// # Example
    ///
    /// ```
    /// # use spinoff::*;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// let sp = Spinner::new_with_stream(Spinners::Dots, "Hello World!", Color::Blue, Streams::Stderr);
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
        let mut stream = Arc::new(Mutex::new(stream.get_stream()));
        let stream_clone = stream.clone();
        // We use atomic bools to make the thread stop itself when the `spinner.stop()` method is called.
        let handle = thread::spawn({
            // Clone the atomic bool so that we can use it in the thread and return the original one later.
            let still_spinning = Arc::clone(&still_spinning);
            let msg = msg.clone();
            move || {
                let spinner_data = SPINNER_FRAMES.get(&spinner_type).unwrap();
                // Iterate over all the frames of the spinner while the atomic bool is true.
                let frames = spinner_data
                    .frames
                    .iter()
                    .cycle()
                    .take_while(|_| still_spinning.load(std::sync::atomic::Ordering::Relaxed));
                // Dynamically delete the last line of the terminal depending on the length of the message + spinner.
                let mut last_length = 0;
                let stream_lock = stream_clone.lock().unwrap();
                for frame in frames {
                    let frame_str = format!(" {} {}", init_color(color, frame), msg);
                    // Get us back to the start of the line.
                    delete_last_line(last_length, &mut stream_lock).unwrap();
                    last_length = frame_str.bytes().len();
                    write!(stream_lock, "{}", frame_str).unwrap();
                    stream_lock.flush().unwrap();

                    thread::sleep(std::time::Duration::from_millis(
                        spinner_data.interval as u64,
                    ));
                }
                delete_last_line(last_length, &mut stream_lock).unwrap();
            }
        });

        // Return a Spinner struct
        Self {
            thread_handle: Some(handle),
            still_spinning,
            msg,
            stream,
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
        // Print message
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
    /// let sp = Spinner::new(Spinners::Dots2, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop_with_message("Bye");
    /// ```
    ///
    pub fn stop_with_message(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        // Put the message over the spinner
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
    /// let sp = Spinner::new(Spinners::Mindblown, "Guess what's coming...", None);
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
    /// let sp = Spinner::new(Spinners::Aesthetic, "Trying to load information...", None);
    /// sleep(Duration::from_millis(800));
    /// sp.success("Success!");
    /// ```
    ///
    pub fn success(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        println!("{} {}", init_color(Some(Color::Green), " ✔"), &msg);
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
    pub fn fail(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        eprintln!("{} {}", init_color(Some(Color::Red), " ✖"), &msg);
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
    pub fn warn(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        println!("{} {}", init_color(Some(Color::Yellow), " ⚠"), &msg);
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
    pub fn info(mut self, msg: StringLiteral) {
        self.stop_spinner_thread();
        println!("{} {}", init_color(Some(Color::Blue), " ℹ"), &msg);
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
    /// let mut sp = Spinner::new(Spinners::Grenade, "Clearing...", None);
    /// sleep(Duration::from_millis(800));
    /// sp.clear();
    /// ```
    ///
    pub fn clear(mut self) {
        self.stop_spinner_thread();
    }

    pub fn update<T, U>(&mut self, spinner: Spinners, msg: T, color: U)
    where 
        T: Into<Cow<'static, str>>, 
        U: Into<Option<Color>>
    {
        let stream_lock = self.stream.lock().unwrap();
        let data = &*stream_lock;
        let _ = std::mem::replace(self, Self::new_with_stream(spinner, msg, color, Streams::Custom(data)));
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
