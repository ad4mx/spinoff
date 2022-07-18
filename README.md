# spinoff
`spinoff` is a simple to use library for displaying spinners in the terminal, with plenty of features and options.

[![Version](https://img.shields.io/crates/v/spinoff.svg)](https://crates.io/crates/spinoff) [![Downloads](https://img.shields.io/crates/d/spinoff)](https://crates.io/crates/spinoff) [![Docs](https://img.shields.io/docsrs/spinoff)](https://docs.rs/spinoff/latest/spinoff/)


<p>
	<br>
	<img width="800" src="assets/index.gif">
	<br>
	<br>
</p>

## Usage

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

let mut spinner = Spinner::new(Spinners::Dots, "Loading...", Some(Color::Blue)); 
sleep(Duration::from_secs(3));
spinner.update(Spinners::Dots2, "Loading...", None);
sleep(Duration::from_secs(3));
spinner.stop_and_persist("👨‍💻", "Done!");
```


## Documentation

* All documentation can be found on the [Docs.rs page](https://docs.rs/spinoff/latest/spinoff/).
* If you want to see all the available `Spinner` options, check the [`Spinners`](src/utils/spinner_enum.rs) enum.

## Examples

To run some of the included examples, use: 
```bash	
cargo run --example all_spinners
```

```bash
cargo run --example simple
```

## Contributing

Any contributions to this crate are highly appreciated. If you have any ideas/suggestions/bug fixes, please open an issue or pull request.

## License

This crate is licensed under the [MIT license](LICENSE).