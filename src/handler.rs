use super::{Request, Response};

/// This trait is used to create routers for the HTTP type.
/// Types that implement the `Handler` trait dispatch requests
/// to the proper route. `hayaku-path` and `hayaku-simple-path`
/// are examples of routers that implement this interface.
pub trait Handler<T: Clone> {
    fn handler(&self, &Request, &mut Response, &T);
}
