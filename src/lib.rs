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

#[deriving(Clone)]
struct Encoded(HashMap<String, Vec<String>>);

#[deriving(Clone)]
pub struct UrlEncoded;

impl UrlEncoded {
    pub fn new() -> UrlEncoded {
        UrlEncoded
    }
}

impl Middleware for UrlEncoded {
    fn enter(&mut self, req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {
        
        let raw_url = url::path_from_str(req.url().unwrap().as_slice());

        let query = match raw_url {
            Ok(e) => {
                e.query
            },
            Err(e) => {
                return Continue;
            }
        };

        alloy.insert::<Encoded>(Encoded(vec_to_hashmap(query.clone())));
        Continue
    }
}

fn vec_to_hashmap(q: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut vec_hashmapped: HashMap<String, Vec<String>> = HashMap::new();
 
    for (k, v) in q.move_iter() {
        vec_hashmapped.find_with_or_insert_with(
            k, v,
            |_, already, new| {
                already.push(new);
            },
            |_, v| vec![v]
        );
    }
    vec_hashmapped
}
