// This example is almost the same as get_request.rs, but uses UrlEncodedBody
extern crate iron;
extern crate urlencoded;

use iron::{Iron, Server, Chain, Request, Response, FromFn};
use iron::{Status, Continue};
use iron::Plugin;
use urlencoded::UrlEncodedBody;
use std::io::net::ip::Ipv4Addr;

fn log_post_data(req: &mut Request, _: &mut Response) -> Status {
    match req.get_ref::<UrlEncodedBody>() {
        Some(ref hashmap) => println!("Parsed POST request body:\n {}", hashmap),
        None => println!("Error, no body found.")
    }

    Continue
}

// Test with `curl -i -X POST "http://localhost:3000/" --data "fruit=apple&name=iron&fruit=pear"`
fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(FromFn::new(log_post_data));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
