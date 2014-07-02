urlencoded [![Build Status](https://secure.travis-ci.org/iron/urlencoded.png?branch=master)](https://travis-ci.org/iron/urlencoded)
====

> URL Encoded middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate iron;
extern crate urlencoded;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Server, Request, Response, Alloy, Chain};

use urlencoded::{UrlEncoded, Encoded};

fn log_hashmap( _ : &mut Request, _ : &mut Response, alloy: &mut Alloy) {
    let hashmap = alloy.find::<Encoded>();
    match hashmap {
        Some(&Encoded(ref encoded)) => println!("Url Encoded:\n {}", encoded),
        None => ()
    }
}

fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(UrlEncoded::new());
    server.chain.link(log_hashmap);
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
```

## Overview

urlencoded is a part of Iron's [core bundle](https://github.com/iron/core).

- Handles url parsing and parses parameters after the `?` into a `HashMap` that maps
`String` representations of keys into a `Vector` of `String` values.
- Values are populated into a `Vector` to ensure that multiple values which are passed
into the url are also stored and assessible. 

## Installation

If you're using a `Cargo` to manage dependencies, just add urlencoded to the toml:

```toml
[dependencies.urlencoded]

git = "https://github.com/iron/urlencoded.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://docs.ironframework.io/urlencoded)

Along with the [online documentation](http://docs.ironframework.io/urlencoded),
you can build a local copy with `make doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
