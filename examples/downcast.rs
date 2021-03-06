#![allow(unstable)]

#[macro_use(match_error)]
extern crate error;

use std::error::Error as StdError;
use std::fmt::Error as FmtError;
use error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Show, PartialEq, Copy)]
pub struct ParseError {
    location: usize,
}

impl StdError for ParseError {
    fn description(&self) -> &str { "Parse Error" }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.description().fmt(f)
    }
}

fn produce_parse_error() -> Box<Error> {
    Box::new(ParseError { location: 7us })
}

fn generic_handler(raw: Box<Error>) {
    (match_error! { &*raw,
        parse => ParseError: {
            assert_eq!(*parse, ParseError { location: 7us })
        }
    }).unwrap()
}

fn main() {
    generic_handler(produce_parse_error())
}

