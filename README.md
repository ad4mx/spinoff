# spinoff
> simple library for displaying spinners in the terminal

[![Version](https://img.shields.io/crates/v/spinoff.svg)](https://crates.io/crates/spinoff) [![Downloads](https://img.shields.io/crates/d/spinoff)](https://crates.io/crates/spinoff) [![Docs](https://img.shields.io/docsrs/spinoff)](https://docs.rs/spinoff/latest/spinoff) [![License](https://img.shields.io/crates/l/spinoff)](https://crates.io/crates/spinoff) ![Actions](https://img.shields.io/github/workflow/status/ad4mx/spinoff/Rust)


![](assets/index.gif)
## 🔨 Install
Add as a dependency to your `Cargo.toml`:

```toml
[dependencies]
spinoff = "0.5.3"
```	

## ⚡ Usage

```rust
use spinoff::{Spinner, Spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let spinner = Spinner::new(Spinners::Dots, "Loading...", Color::Blue); 
sleep(Duration::from_secs(3));
spinner.success("Done!");
```

### Update a spinner

```rust
use spinoff::{Spinner, Spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let mut spinner = Spinner::new(Spinners::Aesthetic, "Loading...", Color::Red); 
sleep(Duration::from_secs(3));
spinner.update(Spinners::Dots2, "Retrying...", None);
sleep(Duration::from_secs(3));
spinner.update_text("Not quite done...");
sleep(Duration::from_secs(1));
spinner.stop()
```

### Stop a spinner and persist a symbol and message

```rust
use spinoff::{Spinner, Spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let spinner = Spinner::new(Spinners::Arc, "Loading...", Color::Green);
sleep(Duration::from_secs(3));
spinner.stop_and_persist("📜", "Task done.");
```

### Specify an output stream

```rust
use spinoff::{Spinner, Spinners, Color, Streams};
use std::thread::sleep;
use std::time::Duration;

let spinner = Spinner::new_with_stream(Spinners::Line, "Loading...", Color::Yellow, Streams::Stderr);
sleep(Duration::from_secs(3));
spinner.stop_and_persist("📜", "Task done.");
```

Other examples can be found in the [documentation](https://docs.rs/spinoff/latest/spinoff/).


## 📖 Documentation

* All relevant documentation can be found on the [Docs.rs page](https://docs.rs/spinoff/latest/spinoff/).
* If you want to see all the available `Spinner` options, check the [`Spinners`](src/spinner_enum.rs) enum.

## ⚙ Examples

To run some of the included examples, use: 
```bash	
cargo run --example all_spinners
```

```bash
cargo run --example simple
```

## 🚧 Contributing

Any contributions to this crate are highly appreciated. If you have any ideas/suggestions/bug fixes, please open an issue or a pull request.
If you like the project, [star this project on GitHub.](https://github.com/ad4mx/spinoff)

## ❗️ Disclaimer

This project is still heavily unstable and is not meant to be used in production. It is still in rapid development and may change without notice.

## 📑 License

This crate is licensed under the [MIT license](LICENSE).
