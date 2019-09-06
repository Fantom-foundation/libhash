libhash
===========
[![Build Status](https://travis-ci.org/Fantom-foundation/libhash.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libhash)

libhash in Rust.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

This crate defines a trait which contains commonly used behaviours for hashing.
An example of an implementation of this trait can be found in the libhash-sha3 crate:
[`libhash-sha3`](https://github.com/Fantom-foundation/libhash-sha3)

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libhash && cd libhash
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```
