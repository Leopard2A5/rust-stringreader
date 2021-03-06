# Rust StringReader

[![Build Status](https://travis-ci.org/Leopard2A5/rust-stringreader.svg?branch=master)](https://travis-ci.org/Leopard2A5/rust-stringreader)

Provides a wrapper for strings so that they can be consumed via the std::io::Read trait. This is especially useful when writing tests.

## Deprecation
This create is deprecated. `std::io::Read` is implemented for `&'a [u8]`, which you can get from a String using the `as_bytes` [method](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes).

## Example

```rust
use std::io::{Read, BufRead, BufReader};
use stringreader::StringReader;

let mut streader = StringReader::new("Line 1\nLine 2");
let mut bufreader = BufReader::new(streader);
let mut buffer = String::new();

bufreader.read_line(&mut buffer).unwrap();
println!("{}", buffer);
```

Prints "Line 1\n".

## Usage

Cargo.toml:
```
[dependencies]
stringreader = "*"
```

lib.rs/main.rs:
```rust
extern crate stringreader;

use stringreader::StringReader;

// ...
let mut reader = StringReader::new("this is a test");
// ...
```
