use cookie::Cookie;
use httbloat;
use super::Status;

use std::default::Default;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub struct Response(httbloat::Response);

impl Default for Response {
    fn default() -> Self {
        Response(httbloat::Response::default())
    }
}

impl Response {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn status(&mut self, status: Status) {
        self.0.status(status);
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.0.add_header(name, value);
    }

    pub fn body(&mut self, body: &[u8]) {
        self.0.body(body);
    }

    pub fn send_file<P: AsRef<Path>>(&mut self, filename: P) -> io::Result<()> {
        let mut file = fs::File::open(filename)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.body(&buf);
        Ok(())
    }

    pub fn redirect<S: Into<String>>(&mut self, status: Status, location: S, data: &[u8]) {
        self.status(status);
        self.add_header("Location".to_string(), location.into());
        self.body(data);
    }

    pub fn set_cookie(&mut self, cookie: &Cookie) {
        // TODO(nokaa): rethink this interface
        let cookie = String::from_utf8(cookie.as_bytes()).unwrap();
        self.add_header("Set-Cookie".to_string(), cookie);
    }

    pub fn into_httbloat_response(self) -> httbloat::Response {
        self.0
    }
}
