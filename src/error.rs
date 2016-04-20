//! Error module
use std::error;
use std::fmt;
use std::io;

use hyper;
use rustc_serialize::json;

/// A set of errors that can occurs when accessing the OpenExchangeRates API.
#[derive(Debug)]
pub enum Error {
    /// Error coming from `hyper` crate.
    Hyper(hyper::Error),
    /// Error coming from `std::io` library.
    Io(io::Error),
    /// Error coming from `rustc_serialize` crate.
    Decode(json::DecoderError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Decode(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Hyper(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Decode(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Hyper(ref err) => write!(f, "Hyper error: {}", err),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Decode(ref err) => write!(f, "Decode error: {}", err),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::Decode(err)
    }
}
