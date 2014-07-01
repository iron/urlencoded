//! Example of using urlencoded
extern crate iron;
extern crate urlencoded;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, ServerT, Request, Response, Alloy, Chain};
use iron::middleware::{Status, Continue, FromFn};

use urlencoded::{UrlEncoded, Encoded};

// urlencoded returns a Hashmap 
// Here we create a function to log the hashmap we are storing in Alloy.
// Alloy is where your middleware can store data and we access it through
// the `find` API exposed by alloy.
fn log_hashmap( _ : &mut Request, _ : &mut Response, alloy: &mut Alloy) -> Status {
    let hashmap = alloy.find::<Encoded>();
    match hashmap {
        Some(&Encoded(ref encoded)) => println!("Url Encoded:\n {}", encoded),
        None => ()
    }
    Continue
}

// test out the server with `curl -i "127.0.0.1:3000/?name=franklin&name=trevor"`
fn main() {
    let mut server: ServerT = Iron::new();
    server.chain.link(UrlEncoded::new());
    server.chain.link(FromFn::new(log_hashmap));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
