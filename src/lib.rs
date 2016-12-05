/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */

#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
extern crate cookie;
extern crate urlencoded;
extern crate multipart;

mod handler;
mod request;
mod response;

pub use handler::Handler;
pub use hyper::Method;
pub use hyper::StatusCode as Status;
pub use hyper::header;
pub use response::Response;
pub use request::Request;

use futures::{Finished, finished};
use hyper::server::{Server, Service};

use std::net::SocketAddr;
use std::sync::Arc;

// TODO(nokaa): We probably want to enforce the Clone trait bound on `T`
// here. We can't do this until https://github.com/rust-lang/rust/issues/21903
// is resovled. This shouldn't be a problem because when we use this type we
// are constraining `T` to be Clone.
pub type RequestHandler<T> = Arc<(Fn(Request, &mut Response, &T)) + Send + Sync>;

#[derive(Clone)]
pub struct Http<T: Clone + Send, H: Clone + Send + Handler<T>> {
    handler: H,
    context: T,
    sanitize_input: bool,
}

impl<T: 'static + Clone + Send, H: 'static + Clone + Send + Handler<T>> Service for Http<T, H> {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Finished<Self::Response, hyper::Error>;

    fn call(&self, req: hyper::server::Request) -> Self::Future {
        // We declare these variables here to satisfy lifetime requirements.
        // Note that as these are both Rc (smart pointers) we can clone them
        // without issue.
        let handler = self.handler.clone();
        let context = self.context.clone();
        let sanitize = self.sanitize_input;
        let req = Request::new(req, sanitize);

        finished({
            let mut res = Response::new();
            handler.handler(req, &mut res, &context);
            res.into_hyper_response()
        })
    }
}

impl<T: 'static + Clone + Send, H: 'static + Clone + Send + Handler<T>> Http<T, H> {
    /// Create a new Http handler
    pub fn new(handler: H, context: T) -> Self {
        Http {
            handler: handler,
            context: context,
            sanitize_input: false,
        }
    }

    /// Calling this method will cause form data to be HTML-escaped
    /// when parsed.
    pub fn sanitize(&mut self) -> &mut Self {
        self.sanitize_input = true;
        self
    }

    /// Run the server
    pub fn listen_and_serve(self, addr: SocketAddr) {
        let server = Server::http(&addr).unwrap();
        let (listener, server) = server.standalone(move || Ok(self.clone())).unwrap();
        server.run();
    }
}
