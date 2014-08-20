//! This example shows how to use urlencoded to parse GET request parameters.
extern crate iron;
extern crate urlencoded;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Server, Request, Response, Chain, Status, Continue, FromFn};
use iron::Plugin;
use urlencoded::UrlEncodedQuery;

fn log_params(req: &mut Request, _ : &mut Response) -> Status {
    // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
    match req.get_ref::<UrlEncodedQuery>() {
        Some(hashmap) => println!("Parsed GET request query string:\n {}", hashmap),
        None => println!("Error, no query string found")
    }

    Continue
}

// Test out the server with `curl -i "http://localhost:3000/?name=franklin&name=trevor"`
fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(FromFn::new(log_params));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
