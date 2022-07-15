//! ## spinoff
//!
//! `spinoff` is a simple library for displaying spinners in the terminal.
//! ### Usage
//!
//! ```rust
//! use spinoff::Spinners;
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! let sp = spinoff::new(Spinners::Dots, "Loading...", None);
//! sleep(Duration::from_millis(800));
//! sp.success("Success!");
//!
//! ```
//! ### Spinners
//! There are over 80+ spinners available in the `Spinners` enum.
//!
//! ### Colors
//! You can also color your spinners without any hassle. Simply pass a color to the `color` option.
//! There are 6 colors available: `blue`, `green`, `red`, `yellow`, `cyan`, `white`.
//! Don't want any of that? Simply pass `None` to the `color` option.
//!
#![allow(clippy::needless_return)]
mod utils;
use crate::utils::printer::{delete_last_line, init_color};
use crate::utils::spinner_data::SPINNER_FRAMES;
pub use crate::utils::spinner_enum::Spinners;
use std::io::{stdout, Write};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
// Using this type for better readability.
type StringLiteral = &'static str;

/// All methods for the spinner are implemented in this struct.
/// This struct has an `Arc<AtomicBool>` field, which is later used in the `stop` type methods to stop the thread printing the spinner.
/// The `msg` field is needed for the `clear` and `stop` methods for knowing how many spaces to print to clean up the previous spinner.

pub struct Spinner {
    still_spinning: Arc<AtomicBool>,
    msg: StringLiteral,
}
/// Create a new spinner.
///
/// # Arguments
/// * `spinner_type` - The spinner to use.
/// * `msg` - The message to display.
/// * `color` - The color of the spinner.
///
/// # Example
/// ```rust
/// use spinoff::Spinners;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// let sp = spinoff::new(Spinners::Dots, "Hello", "blue".into()); // Color can also be `None` or `Some("blue")`
/// sleep(Duration::from_millis(800));
/// sp.clear();
/// ```
/// # Panics
/// * `Invalid color` - If the color is not one of the following: `blue`, `green`, `red`, `yellow`, `cyan`, `white` or `None`.
/// * `Invalid spinner` - If the spinner is not one of those belonging to the `Spinners` enum.
///
/// # Notes
/// * The spinner immediately starts spinning upon creation.
pub fn new(spinner_type: Spinners, msg: StringLiteral, color: Option<StringLiteral>) -> Spinner {
    let spinner_data = SPINNER_FRAMES
        .get(&spinner_type)
        .expect("invalid spinner: expected variant of Spinners enum");
    let mut stdout = stdout();
    let mut i = 0;
    let still_spinning = Arc::new(AtomicBool::new(true));
    // Clone the atomic bool so that we can use it in the thread and return the original one later.
    let still_spinning_cloned = still_spinning.clone();
    // We use atomic bools to make the thread stop itself when the `spinner.stop()` method is called.
    thread::spawn(move || {
        while still_spinning_cloned.load(std::sync::atomic::Ordering::Relaxed) {
            let text = format!(
                "\r{} {}",
                init_color(color, spinner_data.frames[i].to_string()),
                msg
            );
            stdout.write_all(text.as_bytes()).unwrap();
            stdout.flush().unwrap();
            thread::sleep(std::time::Duration::from_millis(
                spinner_data.interval as u64,
            ));
            i = (i + 1) % spinner_data.frames.len();
        }
    });
    // Return a Spinner struct so we can implement methods on it instead of `spinoff::stop()` etc.
    let spinner = Spinner {
        still_spinning,
        msg,
    };
    return spinner;
}

impl Spinner {
    /// Stop the spinner.
    ///
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop();
    /// ```
    /// # Notes
    /// * The spinner will be deleted after this method is called, the message will remain though.
    /// * This method also sets the `still_spinning` atomic bool to `false`, which stops the spinner thread.
    /// * This method cannot be called if the spinner is already stopped.
    ///
    pub fn stop(self) {
        self.still_spinning
            .store(false, std::sync::atomic::Ordering::Relaxed);
        print!("\r");
        println!("{}    ", self.msg);
    }

    /// Stops the spinner and prints a message on a new line.
    ///     
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop_with_message("Bye", "red".into());
    /// ```
    ///
    /// # Panics
    /// * The method will panic if the color is not one of the following: `blue`, `green`, `red`, `yellow`, `cyan`, `white` or `None`.
    ///
    /// # Notes
    /// * This method cannot be called if the spinner is already stopped.
    ///
    pub fn stop_with_message(self, msg: StringLiteral, color: Option<StringLiteral>) {
        self.stop();
        println!("{}", init_color(color, msg.into()));
    }

    /// Deletes the spinner and message and prints a new line with a symbol and message.
    ///
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop_and_persist("🍕", "Pizza!", None);
    /// ```
    ///
    /// # Panics
    /// * The method will panic if the color is not one of the following: `blue`, `green`, `red`, `yellow`, `cyan`, `white` or `None`.
    ///
    /// # Notes
    /// * This method will delete the last line of the terminal, so it is recommended to not print anything in between the spinner and the success message.
    /// * This method cannot be called if the spinner is already stopped.
    pub fn stop_and_persist(self, symbol: StringLiteral, msg: StringLiteral, color: Option<StringLiteral>) {
        self.clear();
        println!("{} {}", init_color(color, symbol.into()), &msg);
    }

    /// Deletes the last line of the terminal and prints a success symbol with a message.
    ///
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///     
    /// let sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.success("Success!");
    /// ```
    ///
    /// # Notes
    /// * This method cannot be called if the spinner is already stopped.
    /// * This method will delete the last line of the terminal, so it is recommended to not print anything in between the spinner and the success message.
    pub fn success(self, msg: StringLiteral) {
        self.stop_and_persist("✔", msg, "green".into());
    }

    /// Deletes the last line of the terminal and prints a failure symbol with a message.
    ///     
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///     
    /// let sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.fail("Failed!");
    /// ```
    ///
    /// # Notes
    /// * This method will delete the last line of the terminal, so it is recommended to not print anything in between the spinner and the failure message.
    /// * This method cannot be called if the spinner is already stopped.
    ///
    pub fn fail(self, msg: StringLiteral) {
        self.stop_and_persist("✖", msg, "red".into());
    }

    /// Deletes the last line of the terminal and prints a warning symbol with a message.
    ///     
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///     
    /// let sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.warn("Look out!");
    /// ```
    ///
    ///
    /// # Notes
    /// * This method will delete the last line of the terminal, so it is recommended to not print anything in between the spinner and the warning message.
    /// * This method cannot be called if the spinner is already stopped.
    pub fn warn(self, msg: StringLiteral) {
        self.stop_and_persist("⚠ ", msg, "yellow".into());
    }

    /// Deletes the last line of the terminal and prints a new spinner.
    ///
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///     
    /// let mut sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp = sp.update(Spinners::Dots2, "Goodbye".into(), None);
    /// sleep(Duration::from_millis(800));
    /// sp.stop();
    /// ```
    ///
    /// # Notes
    /// * This method will delete the last line of the terminal, so it is recommended to not print anything in between the spinner and the new spinner instance.
    /// * This method cannot be called if the spinner is already stopped.
    ///
    pub fn update(self, spinner: Spinners, msg: StringLiteral, color: Option<StringLiteral>) -> Spinner {
        self.still_spinning
            .store(false, std::sync::atomic::Ordering::Relaxed);
        delete_last_line(self.msg);
        self::new(spinner, msg, color)
    }
    /// Deletes the last line of the terminal.
    ///     
    /// # Example
    /// ```rust
    /// use spinoff::Spinners;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let mut sp = spinoff::new(Spinners::Dots, "Hello", None);
    /// sleep(Duration::from_millis(800));
    /// sp.clear();
    /// ```
    ///
    /// # Notes
    /// * This method will delete the last line of the terminal, so it is recommended to not print anything in between the spinner and the `delete` method call.
    /// * This method cannot be called if the spinner is already stopped.
    pub fn clear(self) {
        self.still_spinning
            .store(false, std::sync::atomic::Ordering::Relaxed);
        delete_last_line(self.msg);
    }
}
