termize
====

[![Crates.io](https://img.shields.io/crates/v/termize.svg)](https://crates.io/crates/termize) [![Crates.io](https://img.shields.io/crates/d/termize.svg)](https://crates.io/crates/termize) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/JohnTitor/termize/blob/master/LICENSE-MIT) [![license](http://img.shields.io/badge/license-Apache2.0-blue.svg)](https://github.com/JohnTitor/termize/blob/master/LICENSE-APACHE) [![Actions](https://github.com/JohnTitor/termize/workflows/CI/badge.svg)](https://github.com/JohnTitor/termize/workflows/CI) [![Cirrus CI](https://api.cirrus-ci.com/github/JohnTitor/termize.svg)](https://cirrus-ci.com/github/JohnTitor/termize)

A Rust library to enable getting terminal sizes and dimensions

**This is a fork repository, original is [here](https://github.com/clap-rs/term_size-rs).**

MSRV (Minimal Supported Rust Version): 1.31.1

[Documentation](https://docs.rs/termize)

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
termize = "0.1.0"
```

To get the dimensions of your terminal window, simply use the following:

```rust
fn main() {
  if let Some((w, h)) = termize::dimensions() {
    println!("Width: {}\nHeight: {}", w, h);
  } else {
    println!("Unable to get term size :(");
  }
}
```

## License

Copyright Benjamin Sago, Kevin Knapp, Yuki Okushi, and `term_size` contributors.

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option. Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D
