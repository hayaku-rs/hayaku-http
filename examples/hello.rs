extern crate hayaku_http;

use hayaku_http::{Http, Handler, Request, Response};

#[derive(Copy, Clone)]
struct Router;

impl Handler<()> for Router {
    fn handler(&self, _req: &Request, res: &mut Response, _ctx: &()) {
        res.body(b"hello, world!");
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();

    let router = Router;
    Http::new(router, ()).threads(4).listen_and_serve(addr);
}
