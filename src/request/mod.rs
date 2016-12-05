use futures::{Async, Stream};
use hyper::{self, Method, RequestUri, HttpVersion};
use hyper::header::Headers;
use cookie::Cookie;
use urlencoded::{parse_urlencoded, parse_urlencoded_html_escape};

use std::collections::HashMap;
use std::cell::RefCell;

// mod multipart;

pub struct Request {
    request: hyper::server::Request,
    pub user_data: RefCell<Vec<u8>>,
    form: RefCell<Option<HashMap<String, String>>>,
    sanitize_input: bool,
}

impl Request {
    pub fn new(req: hyper::server::Request, sanitize: bool) -> Self {
        Request {
            request: req,
            user_data: RefCell::new(Vec::new()),
            form: RefCell::new(None),
            sanitize_input: sanitize,
        }
    }

    pub fn method(&self) -> &Method {
        self.request.method()
    }

    pub fn headers(&self) -> &Headers {
        self.request.headers()
    }

    pub fn uri(&self) -> &RequestUri {
        self.request.uri()
    }

    pub fn version(&self) -> &HttpVersion {
        self.request.version()
    }

    pub fn path(&self) -> Option<&str> {
        self.request.path()
    }

    pub fn query(&self) -> Option<&str> {
        self.request.query()
    }

    pub fn form_value<S: Into<String>>(self, key: S) -> Option<String> {
        use std::ops::Deref;
        let key = key.into();

        if *self.form.borrow() == None {
            let mut body = self.request.body();
            match body.poll() {
                Ok(Async::Ready(Some(chunk))) => {
                    let map = if self.sanitize_input {
                        parse_urlencoded_html_escape(chunk.deref())
                    } else {
                        parse_urlencoded(chunk.deref())
                    };
                    let map = match map {
                        Ok(m) => m,
                        Err(e) => {
                            // For now if we can't parse the form we
                            // just return an empty map
                            debug!("Error parsing form: {}", e);
                            HashMap::new()
                        }
                    };
                    *self.form.borrow_mut() = Some(map);
                }
                Ok(Async::Ready(None)) => {
                    *self.form.borrow_mut() = Some(HashMap::new());
                }
                Ok(Async::NotReady) => {
                    *self.form.borrow_mut() = Some(HashMap::new());
                }
                Err(e) => {
                    *self.form.borrow_mut() = Some(HashMap::new());
                }
            }
            /*match *self.body {
                None => return None,
                Some(ref b) => {
                    let body = &b.data[..];
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
            }*/
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
}
/*pub struct Request<'a> {
    pub method: Method,
    pub path: &'a String,
    pub version: &'a enums::Version,
    pub headers: &'a Vec<(enums::Header, String)>,
    pub body: &'a Option<request::Body>,
    pub peer_addr: &'a SocketAddr,
    request: &'a minihttp::Request,
    form: RefCell<Option<HashMap<String, String>>>,
    pub user_data: RefCell<Vec<u8>>,
    sanitize_input: bool,
}

impl<'a> Request<'a> {
    pub fn new(req: &'a minihttp::Request, sanitize: bool) -> Request<'a> {
        Request {
            method: Method::from(&req.method),
            path: &req.path,
            version: &req.version,
            headers: &req.headers,
            body: &req.body,
            peer_addr: &req.peer_addr,
            request: req,
            form: RefCell::new(None),
            user_data: RefCell::new(Vec::new()),
            sanitize_input: sanitize,
        }
    }

    pub fn has_body(&self) -> bool {
        self.request.has_body()
    }

    pub fn host(&self) -> Option<&str> {
        self.request.host()
    }

    pub fn content_type(&self) -> Option<&str> {
        self.request.content_type()
    }

    pub fn content_length(&self) -> Option<u64> {
        self.request.content_length()
    }

    pub fn transfer_encoding(&self) -> Option<&str> {
        self.request.transfer_encoding()
    }

    pub fn form_value<S: Into<String>>(&self, key: S) -> Option<String> {
        let key = key.into();

        if *self.form.borrow() == None {
            match *self.body {
                None => return None,
                Some(ref b) => {
                    let body = &b.data[..];
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
        use minihttp::enums::Header;

        let mut cookies = Vec::new();

        for &(ref header, ref value) in self.headers {
            if let &Header::Raw(ref s) = header {
                if s == "Cookie" {
                    let cookie = Cookie::from_bytes(value.as_bytes());
                    cookies.push(cookie);
                }
            }
        }
        cookies
    }
}*/
