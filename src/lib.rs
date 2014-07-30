#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/urlencoded")]
#![license = "MIT"]

//! Url Encoded middleware for Iron.
//!
//! Parses "url encoded" data from client requests.
//! Capable of parsing both URL query strings and POST request bodies.

// Use rust-url over soon to be deprecated liburl
extern crate url;
extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy, Status, Continue};

use url::form_urlencoded;
use std::collections::HashMap;

/// Stores data extracted from the query string and body of a request in a pair of hashmaps.
///
/// Each map is `Some` only if both of the following conditions are met:
///     1. Parsing of the field is enabled in the `UrlEncodedParser`.
///     2. The query string/body of the request is non-empty.
///
/// The values are stored in a vector so that keys which appear multiple times can map to
/// multple values.
/// e.g. "?a=b&a=c" is stored as `a => vec![b, c]`
#[deriving(Clone)]
pub struct UrlEncodedData {
    // HashMap created from URL query string.
    pub query_string: Option<HashMap<String, Vec<String>>>,

    // HashMap created from request body.
    pub body: Option<HashMap<String, Vec<String>>>
}

/// Middleware which inserts a `UrlEncodedData` object into the alloy for later use.
///
/// Can be configured to parse the request's URL, body or both. Always inserts something.
#[deriving(Clone)]
pub struct UrlEncodedParser {
    parse_url: bool,
    parse_body: bool
}

impl UrlEncodedParser {
    /// Creates a `UrlEncodedParser` which operates on both the URL and body
    /// of a Request. This is less efficient than using `url_only` or `body_only`.
    pub fn new() -> UrlEncodedParser {
        UrlEncodedParser { parse_url: true, parse_body: true }
    }

    /// Creates a `UrlEncodedParser` which operates on the `url` field of a `Request`.
    ///
    /// Useful for parsing GET request query strings.
    pub fn url_only() -> UrlEncodedParser {
        UrlEncodedParser { parse_url: true, parse_body: false }
    }

    /// Creates a `UrlEncodedParser` which operates on the `body` field of a `Request`.
    ///
    /// Useful for parsing the bodies of POST requests.
    pub fn body_only() -> UrlEncodedParser {
        UrlEncodedParser { parse_url: false, parse_body: true }
    }
}

impl Middleware for UrlEncodedParser {
    fn enter(&mut self, req: &mut Request, _ : &mut Response, alloy: &mut Alloy) -> Status {
        let mut result = UrlEncodedData { query_string: None, body: None };

        // Parse the url's query string from the '?' characters onwards, if desired.
        // XXX: Fix this when Request is updated (use url::Url::query)
        if self.parse_url {
            let data = match req.url.as_slice().find('?') {
                Some(i) => req.url.as_slice().slice_from(i + 1),
                None => ""
            };

            result.query_string = create_param_hashmap(data);
        }

        // Parse the request's body if desired
        if self.parse_body {
            result.body = create_param_hashmap(req.body.as_slice());
        }

        // Insert the result into the Alloy
        alloy.insert::<UrlEncodedData>(result);

        Continue
    }
}

/// Parses a urlencoded string into an optional HashMap.
fn create_param_hashmap(data: &str) -> Option<HashMap<String, Vec<String>>> {
    match data {
        "" => None,
        _ => Some(combine_duplicates(form_urlencoded::parse_str(data)))
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
