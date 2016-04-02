extern crate hyper;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::io::{self, Read};

use rustc_serialize::json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Io(io::Error),
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

#[derive(RustcDecodable, Debug)]
pub struct ExchangeRate {
    disclaimer: String,
    license: String,
    timestamp: i64,
    base: String,
    rates: BTreeMap<String, f32>,
}

pub struct Client {
    api_key: &'static str,
}

impl Client {
    pub fn new(api_key: &'static str) -> Client {
        Client { api_key: api_key }
    }
    pub fn get_rate(self) -> Result<ExchangeRate, Error> {
        let client = hyper::Client::new();

        let url = &format!("http://localhost:3000?api_key={}", self.api_key);
        let mut res = try!(client.get(url).send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let decoded: ExchangeRate = try!(json::decode(&body));
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn get_rate_works() {
        let client = Client::new("1234");

        let res = client.get_rate();
        assert!(res.is_ok());

        let rate = res.unwrap();
        assert!(rate.disclaimer.len() > 0);
    }
}
