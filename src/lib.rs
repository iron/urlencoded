//! Url Encoded middleware for Iron
//! 
//! This middleware focuses on parsing the incoming url parameters from client requests.
#![crate_id = "urlencoded"]
#![license = "MIT"]

extern crate url;
extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy, Status, Continue};
use iron::mixin::GetUrl;

use std::collections::HashMap;

/// Stores a `HashMap` of a `String` and a `Vec<Strings>` to address
/// client data that is sent with multiple values for a single key.
#[deriving(Clone)]
pub struct Encoded(pub HashMap<String, Vec<String>>);

/// This middleware is used for parsing url parameters and storing
/// the data as conveniently accessible data `insert`ed into an Alloy. 
#[deriving(Clone)]
pub struct UrlEncoded;

/// Creates a `UrlEncoded` instance to `link` to `server.chain`. Calling the
/// function will `insert` a new `HashMap` into the `alloy`.
impl UrlEncoded {
    pub fn new() -> UrlEncoded {
        UrlEncoded
    }
}

impl Middleware for UrlEncoded {

    fn enter(&mut self, req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {

        let raw_url = req.url();

        let path = match raw_url {
            Some(k) => {
                url::path_from_str(k.as_slice())
            },
            None => {
                return Continue;
            }
        };
        
        let query = match path {
            Ok(e) => {
                e.query
            },
            Err(_) => {
                return Continue;
            }
        };
        alloy.insert::<Encoded>(Encoded(create_hash(query)));
        Continue
    }
}

fn create_hash(q: Vec<(String, String)>) -> HashMap<String, Vec<String>> {

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

#[test]
fn test_create_hash() {
    let my_vec = vec!(("band".to_string(), "arctic monkeys".to_string()),("band".to_string(), "temper trap".to_string()),("color".to_string(),"green".to_string()));
    let answer = create_hash(my_vec);
    let mut control = HashMap::new();
    control.insert("band".to_string(), vec!("arctic monkeys".to_string(), "temper trap".to_string()));
    control.insert("color".to_string(), vec!("green".to_string()));
    assert!(answer==control);
}
