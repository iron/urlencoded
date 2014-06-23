//! Url Encoded middleware for Iron
#![crate_id = "urlencoded"]
#![license = "MIT"]

extern crate url;
extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy};
use iron::mixin::GetUrl;
use iron::middleware::{Status, Continue, Unwind};

use url::from_str;
use std::collections::HashMap;

/// `urlencoded` returns a hashmap that maps a string to a Vec
/// of strings. If there are two values assigned to the same key
/// the user can ieterate through the Vec to access all data.
#[deriving(Clone)]
pub struct Encoded(pub HashMap<String, Vec<String>>);

#[deriving(Clone)]
pub struct UrlEncoded;

impl UrlEncoded {
    pub fn new() -> UrlEncoded {
        UrlEncoded
    }
}

impl Middleware for UrlEncoded {
    fn enter(&mut self, req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {

        let query = match url::path_from_str(req.url().unwrap().as_slice()) {
            Ok(e) => {
                e.query
            },
            Err(e) => {
                return Continue;
            }
        };
        alloy.insert::<Encoded>(Encoded(createHash(query)));
        Continue
    }
}

fn createHash(q: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut hashStrVec: HashMap<String, Vec<String>> = HashMap::new();
 
    for (k, v) in q.move_iter() {
        hashStrVec.find_with_or_insert_with(
            k, v,
            |_, already, new| {
                already.push(new);
            },
            |_, v| vec![v]
        );
    }
    hashStrVec
}
