# spinoff
`spinoff` is a simple to use library for displaying spinners in the terminal, with plenty of features and options.

[![Version](https://img.shields.io/crates/v/spinoff.svg)](https://crates.io/crates/spinoff) [![Downloads](https://img.shields.io/crates/d/spinoff)](https://crates.io/crates/spinoff) [![Docs](https://img.shields.io/docsrs/spinoff)](https://docs.rs/spinoff/latest/spinoff) [![License](https://img.shields.io/crates/l/spinoff)](https://crates.io/crates/spinoff) ![Actions](https://img.shields.io/github/workflow/status/ad4mx/spinoff/Rust)


![til](assets/index.gif)

## ⚡ Usage

```rust
use spinoff::{Spinner, Spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let spinner = Spinner::new(Spinners::Dots, "Loading...", Some(Color::Blue)); 
sleep(Duration::from_secs(3));
spinner.success("Done!");
```

### Update a spinner

```rust
use spinoff::{Spinner, Spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let mut spinner = Spinner::new(Spinners::Dots, "Loading...", Some(Color::Red)); 
sleep(Duration::from_secs(3));
spinner.update(Spinners::Dots2, "Loading...", None);
sleep(Duration::from_secs(3));
spinner.stop()
```

### Stop a spinner and persist a symbol and message

```rust
use spinoff::{Spinner, Spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let mut spinner = Spinner::new(Spinners::Dots, "Loading...", Some(Color::Green));
sleep(Duration::from_secs(3));
spinner.stop_and_persist("📜", "Task done.");
```

Other examples can be found in the [documentation](https://docs.rs/spinoff/latest/spinoff/).


## 📖 Documentation

* All relevant documentation can be found on the [Docs.rs page](https://docs.rs/spinoff/latest/spinoff/).
* If you want to see all the available `Spinner` options, check the [`Spinners`](src/spinner_enum.rs) enum.

## 🔨 Examples

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

## 📑 License

This crate is licensed under the [MIT license](LICENSE).
