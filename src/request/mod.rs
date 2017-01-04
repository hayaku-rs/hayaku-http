use httbloat::{self, Method, Header, Version};
use cookie::Cookie;
use urlencoded::parse_urlencoded;

use std::collections::HashMap;
use std::cell::RefCell;

// mod multipart;

/// An HTTP request.
pub struct Request {
    req: httbloat::Request,
    /// `user_data` is a field that can be used to store arbitrary data.
    /// Example usage of this field would be a router storing the values
    /// of parameters for the requested route. A `Vec<u8>` is used to allow
    /// the largest variety of data to be stored. Complex data types can be
    /// stored using serialization like serde.
    pub user_data: RefCell<Vec<u8>>,
    form: RefCell<Option<HashMap<String, String>>>,
}

impl Request {
    /// Create a new request from an httbloat request.
    pub fn new(req: httbloat::Request) -> Self {
        Request {
            req: req,
            user_data: RefCell::new(Vec::new()),
            form: RefCell::new(None),
        }
    }

    /// Returns the method used by this request.
    pub fn method(&self) -> Method {
        self.req.method()
    }

    /// Returns the HTTP version used by this request.
    pub fn version(&self) -> Version {
        self.req.version()
    }

    /// Returns the requested path.
    pub fn path(&self) -> String {
        self.req.path()
    }

    /// Returns the headers sent with this request.
    pub fn headers(&self) -> Vec<(Header, String)> {
        self.req.headers()
    }

    /// Returns true if this request included a body.
    pub fn has_body(&self) -> bool {
        self.req.has_body()
    }

    /// Returns the body of this request if one was sent.
    pub fn body(&self) -> Option<&[u8]> {
        self.req.body()
    }

    /// Attempts to retrieve values from a form sent with this request.
    /// A useful way to work with this method is to call
    /// `req.form_value("key").unwrap_or(String::new())`.
    pub fn form_value<S: Into<String>>(&self, key: S) -> Option<String> {
        let key = key.into();

        if *self.form.borrow() == None {
            match self.req.body() {
                None => return None,
                Some(body) => {
                    info!("Request body: {:?}", body);
                    let m = match parse_urlencoded(body) {
                        Ok(m) => m,
                        Err(e) => {
                            // For now if we can't parse the form we
                            // just return an empty map
                            debug!("Error parsing form: {}", e);
                            HashMap::new()
                        }
                    };
                    *self.form.borrow_mut() = Some(m);
                }
            }
        }

        match *self.form.borrow() {
            Some(ref map) => {
                match map.get(&key) {
                    None => None,
                    Some(s) => Some(s.clone()),
                }
            }
            None => unimplemented!(),
        }
    }

    /// Returns the cookies sent with this request.
    pub fn get_cookies(&self) -> Vec<Cookie> {
        use super::Header;

        let mut cookies = Vec::new();

        for (ref header, ref value) in self.req.headers() {
            if let Header::Raw(ref s) = *header {
                if s == "Cookie" {
                    let cookie = Cookie::from_bytes(value.as_bytes());
                    cookies.push(cookie);
                }
            }
        }
        cookies
    }
}
