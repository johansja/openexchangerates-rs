//! Error module
use std::error;
use std::fmt;
use std::io;

use hyper;
use serde_json;

/// A set of errors that can occurs when accessing the OpenExchangeRates API.
#[derive(Debug)]
pub enum Error {
    /// Error coming from `hyper` crate.
    Hyper(hyper::Error),
    /// Error coming from `std::io` library.
    Io(io::Error),
    /// Error comming from `serde_json` crate.
    SerdeJson(serde_json::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::SerdeJson(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Hyper(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::SerdeJson(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Hyper(ref err) => write!(f, "Hyper error: {}", err),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::SerdeJson(ref err) => write!(f, "Serde JSON error: {}", err),
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}