use httbloat::{self, Method, Header, Version};
use cookie::Cookie;
use urlencoded::{parse_urlencoded, parse_urlencoded_html_escape};

use std::collections::HashMap;
use std::cell::RefCell;

// mod multipart;

pub struct Request {
    req: httbloat::Request,
    pub user_data: RefCell<Vec<u8>>,
    form: RefCell<Option<HashMap<String, String>>>,
    sanitize_input: bool,
}

impl Request {
    pub fn new(req: httbloat::Request, sanitize: bool) -> Self {
        Request {
            req: req,
            user_data: RefCell::new(Vec::new()),
            form: RefCell::new(None),
            sanitize_input: sanitize,
        }
    }

    pub fn method(&self) -> Method {
        self.req.method()
    }

    pub fn version(&self) -> Version {
        self.req.version()
    }

    pub fn path(&self) -> String {
        self.req.path()
    }

    pub fn headers(&self) -> Vec<(Header, String)> {
        self.req.headers()
    }

    pub fn has_body(&self) -> bool {
        self.req.has_body()
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.req.body()
    }

    pub fn form_value<S: Into<String>>(&self, key: S) -> Option<String> {
        let key = key.into();

        if *self.form.borrow() == None {
            match self.req.body() {
                None => return None,
                Some(body) => {
                    info!("Request body: {:?}", body);
                    let m = if self.sanitize_input {
                        parse_urlencoded_html_escape(body)
                    } else {
                        parse_urlencoded(body)
                    };
                    let m = match m {
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

    pub fn get_cookies(&self) -> Vec<Cookie> {
        use super::Header;

        let mut cookies = Vec::new();

        for (ref header, ref value) in self.req.headers() {
            if let &Header::Raw(ref s) = header {
                if s == "Cookie" {
                    let cookie = Cookie::from_bytes(value.as_bytes());
                    cookies.push(cookie);
                }
            }
        }
        cookies
    }
}
