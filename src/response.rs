use cookie::Cookie;
use httbloat;
use super::Status;

use std::default::Default;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

/// An HTTP response.
pub struct Response(httbloat::Response);

impl Default for Response {
    fn default() -> Self {
        Response(httbloat::Response::default())
    }
}

impl Response {
    /// Creates a new response with status `200 OK`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the status for this response.
    pub fn status(&mut self, status: Status) {
        self.0.status(status);
    }

    /// Adds a header with `name` and `value` to this response.
    pub fn add_header(&mut self, name: String, value: String) {
        self.0.add_header(name, value);
    }

    /// Sets the slice `body` as the body of this response.
    pub fn body(&mut self, body: &[u8]) {
        self.0.body(body);
    }

    /// Sets the contents of file `filename` as the body of this response.
    // TODO(nokaa): Utilize the `send_file` syscall here if possible
    pub fn send_file<P: AsRef<Path>>(&mut self, filename: P) -> io::Result<()> {
        let mut file = fs::File::open(filename)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.body(&buf);
        Ok(())
    }

    /// Redirects this response to `location` with `status`. `data` can be
    /// an empty slice, but it is better to include a small message such as
    /// `Redirecting your request to location`.
    pub fn redirect<S: Into<String>>(&mut self, status: Status, location: S, data: &[u8]) {
        self.status(status);
        self.add_header("Location".to_string(), location.into());
        self.body(data);
    }

    /// Sets the given cookie.
    pub fn set_cookie(&mut self, cookie: &Cookie) {
        // TODO(nokaa): rethink this interface
        let cookie = String::from_utf8(cookie.as_bytes()).unwrap();
        self.add_header("Set-Cookie".to_string(), cookie);
    }

    /// Convert this response into an httbloat Response.
    pub fn into_httbloat_response(self) -> httbloat::Response {
        self.0
    }
}
