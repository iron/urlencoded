// This example is almost the same as get_request.rs, but uses UrlEncodedData::body
extern crate iron;
extern crate urlencoded;

use iron::{Iron, Server, Chain, Request, Response, Alloy, FromFn};
use iron::{Status, Continue};
use urlencoded::{UrlEncodedParser, UrlEncodedData};
use std::io::net::ip::Ipv4Addr;

fn log_post_data(_: &mut Request, _: &mut Response, alloy: &mut Alloy) -> Status {
    let data = alloy.find::<UrlEncodedData>().unwrap();

    match data.body {
        Some(ref hashmap) => println!("Parsed POST request body:\n {}", hashmap),
        None => println!("Error, no body found.")
    }

    Continue
}

// Test with `curl -i -X POST "http://localhost:3000/" --data "fruit=apple&name=iron&fruit=pear"`
fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(UrlEncodedParser::body_only());
    server.chain.link(FromFn::new(log_post_data));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
