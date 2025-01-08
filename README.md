# spinoff
> an easy to use, robust library for displaying spinners in the terminal

[![Version](https://img.shields.io/crates/v/spinoff.svg)](https://crates.io/crates/spinoff) [![Downloads](https://img.shields.io/crates/d/spinoff)](https://crates.io/crates/spinoff) [![Docs](https://img.shields.io/docsrs/spinoff)](https://docs.rs/spinoff/latest/spinoff) [![License](https://img.shields.io/crates/l/spinoff)](https://crates.io/crates/spinoff) ![Actions](https://img.shields.io/github/actions/workflow/status/ad4mx/spinoff/rust.yml?branch=main)


![](assets/index.gif)
## 🔨 Install
Add as a dependency to your `Cargo.toml`:

```toml
[dependencies]
spinoff = "0.8.0"
```	

## ⚡ Usage

```rust
use spinoff::{Spinner, spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let mut spinner = Spinner::new(spinners::Dots, "Loading...", Color::Blue); 
sleep(Duration::from_secs(3));
spinner.success("Done!");
```

### Update a spinner

```rust
use spinoff::{Spinner, spinners, Color};
use std::thread::sleep;
use std::time::Duration;

let mut spinner = Spinner::new(spinners::Aesthetic, "Loading...", Color::Red); 
sleep(Duration::from_secs(3));
spinner.update(spinners::Dots2, "Retrying...", None);
sleep(Duration::from_secs(3));
spinner.stop()
```

### Specify an output stream

```rust
use spinoff::{Spinner, spinners, Color, Streams};
use std::thread::sleep;
use std::time::Duration;

let mut spinner = Spinner::new_with_stream(spinners::Line, "Loading...", Color::Yellow, Streams::Stderr);
sleep(Duration::from_secs(3));
spinner.stop_and_persist("📜", "Task done.");
```

## 💫 Spinners

`spinoff` includes over 80+ spinner variants out of the box. 
All spinner variants are treated as features that can be enabled or disabled. By default, all of them are enabled for ease of use.
To disable/enable variants, you will have to edit your `cargo.toml` file:

```toml
[dependencies]
spinoff = { version = "0.8.0", features = ["dots", "arc", "line"] }
```

Any suggestions for new spinner variants are welcome. 

### Creating your own spinner
You can create your own spinner using the `spinner!` macro:

```rust
use spinoff::*;
use std::thread::sleep;
use std::time::Duration;

let frames = spinner!([">", ">>", ">>>"], 100);
let mut sp = Spinner::new(frames, "Hello World!", None);
sleep(Duration::from_millis(800));
sp.stop();
```

### Multiline messages
`spinoff` doesn't support spinners with multiline text out of the box. If you want to use it in your project, please look at [#27](https://github.com/ad4mx/spinoff/issues/27).

## ❗Note for Windows Users
For colors to work properly, you need to add a few extra lines to your code: 
```rust
use colored::control
control::set_virtual_terminal(true).unwrap();
```

## 📖 Documentation

* All relevant documentation can be found on the [Docs.rs page](https://docs.rs/spinoff/latest/spinoff/).
* If you want to see all the available `spinner` options, refer to [the `spinner` module](https://docs.rs/spinoff/0.7.0/spinoff/spinners/index.html).

## ⚙ Examples

```bash
cargo run --example simple
```
```bash
cargo run --example stream
```
```bash
cargo run --example stop_and_persist
```
Other examples can be found in the [documentation](https://docs.rs/spinoff/latest/spinoff/).
## 🚧 Contributing

Any contributions to this crate are highly appreciated. If you have any ideas/suggestions/bug fixes, please open an [issue](https://github.com/ad4mx/spinoff/issues) or a [pull request](https://github.com/ad4mx/spinoff/pulls).
If you like the project, [star this project on GitHub.](https://github.com/ad4mx/spinoff)

## 📑 License

This crate is licensed under the [MIT license](LICENSE).
