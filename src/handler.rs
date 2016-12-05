use super::{Request, Response};

pub trait Handler<T: Clone> {
    fn handler(&self, Request, &mut Response, &T);
}
