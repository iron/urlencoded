urlencoded [![Build Status](https://secure.travis-ci.org/iron/urlencoded.png?branch=master)](https://travis-ci.org/iron/urlencoded)
====

> URL Encoded middleware for the [Iron](https://github.com/iron/iron) web framework.  
> Decode URL Encoded data from GET request queries and POST request bodies.

## Example

This example shows how to use urlencoded to parse GET request parameters.

```rust
extern crate iron;
extern crate urlencoded;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Server, Request, Response, Alloy, Chain, Status, Continue, FromFn};
use urlencoded::{UrlEncodedParser, UrlEncodedData};

// The UrlEncodedParser middleware inserts a UrlEncodedData object into the Alloy.
// The .query_string field of the UrlEncodedData object contains an optional hashmap
// which maps values from the URL's query string onto a vector of values.
// This function prints this hashmap to the console.
fn log_params(_ : &mut Request, _ : &mut Response, alloy: &mut Alloy) -> Status {
    // Extract the parsed data (this always succeeds, because something is always inserted).
    let data = alloy.find::<UrlEncodedData>().unwrap();

    // Extract the relevant hashmap from the parsed data.
    match data.query_string {
        Some(ref hashmap) => println!("Parsed GET request query string:\n {}", hashmap),
        None => println!("Error, no query string found")
    }

    Continue
}

// Test out the server with `curl -i "http://localhost:3000/?name=franklin&name=trevor"`
fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(UrlEncodedParser::url_only());
    server.chain.link(FromFn::new(log_params));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
```

## Overview

urlencoded is a part of Iron's [core bundle](https://github.com/iron/core).

- Parses a URL query string into a `HashMap`s that maps `String` representations
of keys onto a `Vec` of `String` values.
- Values are stored in a `Vec` to ensure that no information is lost if a key appears multiple times.
The query string `a=b&a=c` will result in a mapping from `a` to `[b, c]`.
- Parses POST request bodies for web form data (MIME type: `application/x-www-form-urlencoded`).

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
