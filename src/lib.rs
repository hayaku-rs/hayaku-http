/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */

#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_proto;
extern crate tokio_service;
extern crate httbloat;
extern crate cookie;
extern crate urlencoded;
extern crate multipart;

mod handler;
mod request;
mod response;

pub use handler::Handler;
pub use httbloat::Method;
pub use httbloat::Status;
pub use httbloat::Header;
pub use response::Response;
pub use request::Request;

use futures::{Finished, finished};
use tokio_proto::TcpServer;
use tokio_service::Service;

use std::net::SocketAddr;
use std::io;
use std::sync::Arc;

// TODO(nokaa): We probably want to enforce the Clone trait bound on `T`
// here. We can't do this until https://github.com/rust-lang/rust/issues/21903
// is resovled. This shouldn't be a problem because when we use this type we
// are constraining `T` to be Clone.

/// The type used by all request Handlers.
pub type RequestHandler<T> = Arc<(Fn(&Request, &mut Response, &T)) + Send + Sync>;

/// An HTTP server.
#[derive(Clone)]
pub struct Http<T: Clone + Send, H: Clone + Send + Handler<T>> {
    handler: H,
    context: T,
    num_threads: usize,
}

impl<T: 'static + Clone + Send, H: 'static + Clone + Send + Handler<T>> Service for Http<T, H> {
    type Request = httbloat::Request;
    type Response = httbloat::Response;
    type Error = io::Error;
    type Future = Finished<Self::Response, Self::Error>;

    fn call(&self, req: httbloat::Request) -> Self::Future {
        // We declare these variables here to satisfy lifetime requirements.
        // Note that as these are both Rc (smart pointers) we can clone them
        // without issue.
        let handler = self.handler.clone();
        let context = self.context.clone();
        let req = Request::new(req);

        finished({
            let mut res = Response::new();
            handler.handler(&req, &mut res, &context);
            res.into_httbloat_response()
        })
    }
}

impl<T: 'static + Clone + Send + Sync, H: 'static + Clone + Send + Sync + Handler<T>> Http<T, H> {
    /// Create a new Http server. `handler` is a router which
    /// dispatches all requests. `context` is shared state for the server
    /// usable by all requests.
    pub fn new(handler: H, context: T) -> Self {
        Http {
            handler: handler,
            context: context,
            num_threads: 1,
        }
    }

    /// Sets the number of event loops to run. You probably do not want
    /// to set this higher than the number of CPUs on your machine.
    pub fn threads(mut self, threads: usize) -> Self {
        self.num_threads = threads;
        self
    }

    /// Run the server on `addr`.
    pub fn listen_and_serve(self, addr: SocketAddr) {
        let mut srv = TcpServer::new(httbloat::Http, addr);
        srv.threads(self.num_threads);
        srv.serve(move || Ok(self.clone()));
    }
}
