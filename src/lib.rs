#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! A generic, extendable Error type.

extern crate convertible;

use convertible::Convertible;
use std::intrinsics::TypeId;
use std::any::Any;
use std::fmt::{Show, Formatter, FormatError};

pub use self::abstract::AbstractError;

mod abstract;

pub struct RawError<Mark> {
    pub description: Option<&'static str>,
    pub details: Option<String>,
    pub extensions: Option<Box<Any>>,
    pub cause: Option<Box<AbstractError>>
}

impl<T: 'static> RawError<T> {
    pub fn is<R: 'static>(&self) -> bool { TypeId::of::<T>() == TypeId::of::<R>() }

    pub fn to_abstract(self) -> Box<AbstractError> { box self as Box<AbstractError> }
}

pub trait Error<T: 'static>: Convertible<RawError<T>> {
    fn as_raw(&self) -> RawError<T>;

    fn is<R: 'static>(&self) -> bool { TypeId::of::<T>() == TypeId::of::<R>() }
}

impl<O: 'static, E: Error<O>> Convertible<E> for RawError<O> {
    fn convert(err: &E) -> Option<RawError<O>> { Some(err.as_raw()) }
}

impl<T> Show for RawError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "RawError {{ description: {}, details: {}, cause: {} }}", &self.description, &self.details, &self.cause)
    }
}

#[cfg(test)]
mod test {
    use super::{RawError, Error};
    use convertible::{Convertible, Raw};

    #[deriving(Show)]
    pub struct ParseErrorMarker;

    #[deriving(Show, PartialEq)]
    pub struct ParseError {
        location: uint,
    }

    impl<T: 'static> Convertible<RawError<T>> for ParseError {
        fn convert(err: &RawError<T>) -> Option<ParseError> {
            if err.is::<ParseErrorMarker>() {
                Some(ParseError {
                    location: 7u,
                })
            } else {
                None
            }
        }
    }

    impl Error<ParseErrorMarker> for ParseError {
        fn as_raw(&self) -> RawError<ParseErrorMarker> {
            RawError {
                description: Some("Parse-Error"),
                details: None,
                extensions: None,
                cause: None
            }
        }
    }

    #[test] fn test_convert_from_raw() {
        let raw: RawError<ParseErrorMarker> = RawError {
            description: None,
            details: None,
            extensions: None,
            cause: None
        };

        let parse = raw.to::<ParseError>().unwrap();

        assert_eq!(parse, ParseError { location: 7u });
    }

    #[test] fn test_convert_to_raw() {
        let parse = ParseError { location: 10u };
        assert_eq!(parse.to::<RawError<ParseErrorMarker>>().unwrap().description, Some("Parse-Error"));
    }

    #[test] fn test_generic() {
        fn produce_parse_error() -> RawError<ParseErrorMarker> {
            RawError {
                description: None,
                details: None,
                extensions: None,
                cause: None
            }
        }

        fn generic_handler<T: 'static>(raw: RawError<T>) {
            let parse = raw.to::<ParseError>().unwrap();
            assert_eq!(parse, ParseError { location: 7u });
        }

        generic_handler(produce_parse_error())
    }
}

