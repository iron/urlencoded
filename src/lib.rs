//! URL Encoded Plugin for Iron.
//!
//! Parses "url encoded" data from client requests.
//! Capable of parsing both URL query strings and POST request bodies.

extern crate iron;
extern crate bodyparser;
extern crate url;
extern crate plugin;

use iron::prelude::*;
use iron::typemap::Key;

use url::form_urlencoded;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::fmt;
use std::error::Error as StdError;

/// Plugin for `Request` that extracts URL encoded data from the URL query string.
///
/// Use it like this: `req.get_ref::<UrlEncodedQuery>()`
pub struct UrlEncodedQuery;

/// Plugin for `Request` that extracts URL encoded data from the request body.
///
/// Use it like this: `req.get_ref::<UrlEncodedBody>()`
pub struct UrlEncodedBody;

/// An error representing the two possible errors that can occur during URL decoding.
///
/// The first and probably most common one is for the query to be empty,
/// and that goes for both body and url queries.
///
/// The second type of error that can occur is that something goes wrong
/// when parsing the request body.
#[derive(Debug)]
pub enum UrlDecodingError{
    /// An error parsing the request body
    BodyError(bodyparser::BodyError),
    /// An empty query string, either in body or url query
    EmptyQuery
}

pub use UrlDecodingError::*;

impl fmt::Display for UrlDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}

impl StdError for UrlDecodingError {
    fn description(&self) -> &str {
        match *self {
            BodyError(ref err) => err.description(),
            EmptyQuery => "Expected query, found empty string"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            BodyError(ref err) => Some(err),
            _ => None
        }
    }
}

/// Hashmap mapping strings to vectors of strings.
pub type QueryMap = HashMap<String, Vec<String>>;
/// Result type for decoding query parameters.
pub type QueryResult = Result<QueryMap, UrlDecodingError>;

impl Key for UrlEncodedBody {
    type Value = QueryMap;
}
impl Key for UrlEncodedQuery {
    type Value = QueryMap;
}

impl<'a, 'b> plugin::Plugin<Request<'a, 'b>> for UrlEncodedQuery {
    type Error = UrlDecodingError;

    fn eval(req: &mut Request) -> QueryResult {
        match req.url.query() {
            Some(ref query) => create_param_hashmap(&query),
            None => Err(UrlDecodingError::EmptyQuery)
        }
    }
}

impl<'a, 'b> plugin::Plugin<Request<'a, 'b>> for UrlEncodedBody {
    type Error = UrlDecodingError;

    fn eval(req: &mut Request) -> QueryResult {
        req.get::<bodyparser::Raw>()
            .map(|x| x.unwrap_or("".to_string()))
            .map_err(|e| UrlDecodingError::BodyError(e))
            .and_then(|x| create_param_hashmap(&x))
    }
}

/// Parse a urlencoded string into an optional HashMap.
fn create_param_hashmap(data: &str) -> QueryResult {
    match data {
        "" => Err(UrlDecodingError::EmptyQuery),
        _ => Ok(combine_duplicates(form_urlencoded::parse(data.as_bytes()).into_owned()))
    }
}

/// Convert a list of (key, value) pairs into a hashmap with vector values.
fn combine_duplicates<I: Iterator<Item=(String, String)>>(collection: I) -> QueryMap {
    let mut deduplicated: QueryMap = HashMap::new();

    for (k, v) in collection {
        match deduplicated.entry(k) {
            // Already a Vec here, push onto it
            Occupied(entry) => { entry.into_mut().push(v); },

            // No value, create a one-element Vec.
            Vacant(entry) => { entry.insert(vec![v]); },
        };
    }

    deduplicated
}

#[test]
fn test_combine_duplicates() {
    let my_vec = vec![("band".to_string(), "arctic monkeys".to_string()),
                      ("band".to_string(), "temper trap".to_string()),
                      ("color".to_string(),"green".to_string())];
    let answer = combine_duplicates(my_vec.into_iter());
    let mut control = HashMap::new();
    control.insert("band".to_string(),
                   vec!["arctic monkeys".to_string(), "temper trap".to_string()]);
    control.insert("color".to_string(), vec!["green".to_string()]);
    assert_eq!(answer, control);
}
