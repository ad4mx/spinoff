# spinoff
`spinoff` is a simple to use library for displaying spinners in the terminal, with plenty of features and options.

[![Version](https://img.shields.io/crates/v/spinoff.svg)](https://crates.io/crates/spinoff) [![Downloads](https://img.shields.io/crates/d/spinoff)](https://crates.io/crates/spinoff) [![License](https://img.shields.io/crates/l/spinoff.svg)](https://crates.io/crates/spinoff)

## Usage

```rust
use spinoff::Spinners;
use std::thread::sleep;
use std::time::Duration;

let spinner = spinoff::new(Spinners::Dots, "Loading...", "Blue".into()); // Can also be Some("blue") or None
sleep(Duration::from_secs(3));
spinner.success("Done!");
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