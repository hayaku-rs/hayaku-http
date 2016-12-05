use cookie::Cookie;
use hyper;
use hyper::header::{self, Header, Headers};
use super::Status;

use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub struct Response {
    status: Status,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: Status::Ok,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn header<H: Header>(&mut self, header: H) {
        self.headers.set(header);
    }

    pub fn headers(&mut self, headers: Headers) {
        self.headers = headers;
    }

    pub fn body<T: Into<Vec<u8>>>(&mut self, body: T) {
        self.body = Some(body.into());
    }

    // TODO(nokaa): don't clone here, that's really expensive
    pub fn into_hyper_response(self) -> hyper::server::Response {
        let status = self.status;
        let headers = self.headers;
        let hyper_res = hyper::server::Response::new()
            .status(status)
            .headers(headers);
        if let Some(body) = self.body {
            hyper_res.body(body)
        } else {
            hyper_res
        }
    }

    pub fn send_file<P: AsRef<Path>>(&mut self, filename: P) -> io::Result<()> {
        let mut file = fs::File::open(filename)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.body(buf);
        Ok(())
    }

    pub fn redirect<S: Into<String>>(&mut self, status: Status, location: S, data: &[u8]) {
        self.status(status);
        self.header(header::Location(location.into()));
        self.body(data);
    }

    /*pub fn set_cookie(&mut self, cookie: &Cookie) {
        let cookie = cookie.as_bytes();
        self.header(("Set-Cookie", &cookie));
    }*/
}
