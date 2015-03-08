// This example is almost the same as get_request.rs, but uses UrlEncodedBody
extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

fn log_post_data(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => println!("Parsed POST request body:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
    };

    Ok(Response::with((status::Ok, "Hello!")))
}

// Test with `curl -i -X POST "http://localhost:3000/" --data "fruit=apple&name=iron&fruit=pear"`
fn main() {
    Iron::new(log_post_data).http("127.0.0.1:3000").unwrap();
}
