termize
====

[![Crates.io](https://img.shields.io/crates/v/termize.svg)](https://crates.io/crates/termize) [![Crates.io](https://img.shields.io/crates/d/termize.svg)](https://crates.io/crates/termize) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/JohnTitor/termize/blob/master/LICENSE-MIT) [![license](http://img.shields.io/badge/license-Apache2.0-blue.svg)](https://github.com/JohnTitor/termize/blob/master/LICENSE-APACHE) [![CI](https://github.com/JohnTitor/termize/workflows/CI/badge.svg)](https://github.com/JohnTitor/termize/workflows/CI)

A Rust library to enable getting terminal sizes and dimensions

**This is a fork repository, original is [here](https://github.com/clap-rs/term_size-rs).**

MSRV (Minimal Supported Rust Version): 1.20.0

[Documentation](https://docs.rs/termize)

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
termize = "1"
```

Next, add this to your crate root:

```rust
extern crate termize;
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

Copyright Benjamin Sago, Kevin Knapp, and `term_size` contributors.

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

## Breaking Changes

`termize` takes a similar policy to Rust and will bump the major version number upon breaking changes with only the following exceptions:

* The breaking change is to fix a security concern
* The breaking change is to be fixing a bug (i.e. relying on a bug as a feature)
* The breaking change is a feature isn't used in the wild, or all users of said feature have given approval prior to the change
