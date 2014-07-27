#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/urlencoded")]
#![license = "MIT"]

//! Url Encoded middleware for Iron
//!
//! Parses url parameters from client requests.

extern crate url;
extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy, Status, Continue};

use std::collections::HashMap;

/// Stores decoded key-value pairs.
///
/// Multiple assignment as in `?a=b&a=c` is stored
/// as `a => vec![b, c]`
#[deriving(Clone)]
pub struct Encoded(pub HashMap<String, Vec<String>>);

/// This middleware is used for parsing url parameters and storing
/// the data as conveniently accessible data `insert`ed into an Alloy.
#[deriving(Clone)]
pub struct UrlEncoded;

/// Creates a `UrlEncoded` decoder
impl UrlEncoded {
    pub fn new() -> UrlEncoded { UrlEncoded }
}

impl Middleware for UrlEncoded {

    fn enter(&mut self, req: &mut Request, _ : &mut Response, alloy: &mut Alloy) -> Status {
        match url::path_from_str(req.url.as_slice()) {
            Ok(parsed) => {
                alloy.insert::<Encoded>(Encoded(combine_duplicates(parsed.query)));
                Continue
            },
            _ => Continue
        }
    }
}

fn combine_duplicates(q: Vec<(String, String)>) -> HashMap<String, Vec<String>> {

    let mut deduplicated: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in q.move_iter() {
        deduplicated.find_with_or_insert_with(
            k, v,
            // Already a Vec here, push onto it
            |_, already, new| { already.push(new); },
            // No value, create a one-element Vec.
            |_, v| vec![v]
        );
    }

    deduplicated
}

#[test]
fn test_combine_duplicates() {
    let my_vec = vec!(("band".to_string(), "arctic monkeys".to_string()),
                      ("band".to_string(), "temper trap".to_string()),
                      ("color".to_string(),"green".to_string()));
    let answer = combine_duplicates(my_vec);
    let mut control = HashMap::new();
    control.insert("band".to_string(),
                   vec!("arctic monkeys".to_string(), "temper trap".to_string()));
    control.insert("color".to_string(), vec!("green".to_string()));
    assert_eq!(answer, control);
}

