#![crate_name = "urlencoded"]

#![feature(default_type_params)]

//! URL Encoded Plugin for Iron.
//!
//! Parses "url encoded" data from client requests.
//! Capable of parsing both URL query strings and POST request bodies.

extern crate iron;
extern crate url;
extern crate serialize;

extern crate plugin;
extern crate typemap;

use iron::Request;

use url::form_urlencoded;
use std::collections::hash_map::{HashMap, Occupied, Vacant};
use std::str;

use plugin::{PluginFor, Phantom};
use typemap::Assoc;

/// Plugin for `Request` that extracts URL encoded data from the URL query string.
///
/// Use it like this: `req.get_ref::<UrlEncodedQuery>()`
pub struct UrlEncodedQuery;

/// Plugin for `Request` that extracts URL encoded data from the request body.
///
/// Use it like this: `req.get_ref::<UrlEncodedBody>()`
pub struct UrlEncodedBody;

/// Hashmap mapping strings to vectors of strings.
pub type QueryMap = HashMap<String, Vec<String>>;

impl Assoc<QueryMap> for UrlEncodedQuery {}
impl Assoc<QueryMap> for UrlEncodedBody {}

impl PluginFor<Request, QueryMap> for UrlEncodedQuery {
    fn eval(req: &mut Request, _: Phantom<UrlEncodedQuery>) -> Option<QueryMap> {
        match req.url.query {
            Some(ref query) => create_param_hashmap(query.as_slice()),
            None => None
        }
    }
}


impl PluginFor<Request, HashMap<String, Vec<String>>> for UrlEncodedBody {
    fn eval(req: &mut Request, _: Phantom<UrlEncodedBody>) -> Option<QueryMap> {
        str::from_utf8(req.body.as_slice()).and_then(create_param_hashmap)
    }
}

/// Parse a urlencoded string into an optional HashMap.
fn create_param_hashmap(data: &str) -> Option<HashMap<String, Vec<String>>> {
    match data {
        "" => None,
        _ => Some(combine_duplicates(form_urlencoded::parse_str(data)))
    }
}

/// Convert a list of (key, value) pairs into a hashmap with vector values.
fn combine_duplicates(q: Vec<(String, String)>) -> HashMap<String, Vec<String>> {

    let mut deduplicated: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in q.into_iter() {
        match deduplicated.entry(k) {
            // Already a Vec here, push onto it
            Occupied(entry) => { entry.into_mut().push(v); },

            // No value, create a one-element Vec.
            Vacant(entry) => { entry.set(vec![v]); }
        };
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
