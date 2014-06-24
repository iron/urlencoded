//! Example of using urlencoded
extern crate iron;
extern crate urlencoded;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, ServerT, Request, Response, Alloy};

use urlencoded::{UrlEncoded, Encoded};

// urlencoded returns a Hashmap 
// Here we create a function to log the hashmap we are storing in Alloy.
// Alloy is where your middleware can store data and we access it through
// the `find` API exposed by alloy.
fn log_hashmap(req: &mut Request, res: &mut Response, alloy: &mut Alloy) {
    let hashmap = alloy.find::<Encoded>();
    match hashmap {
        Some(&Encoded(ref encoded)) => println!("Url Encoded:\n {}", encoded),
        None => ()
    }
}

// test out the server with `curl -i "127.0.0.1:3000/?name=franklin&name=trevor"`
fn main() {
    let mut server: ServerT = Iron::new();
    server.link(UrlEncoded::new());
    server.link(log_hashmap);
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
