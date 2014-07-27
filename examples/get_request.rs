//! This example shows how to use urlencoded to parse GET request parameters.
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
