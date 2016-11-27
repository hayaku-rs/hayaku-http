use multipart::server::HttpRequest;

use super::Request;

use std::io::Read;

impl<'a> HttpRequest for &'a mut Request<'a> {
    type Body = &'a mut Read;

    // TODO(nokaa): don't use unwrap
    fn multipart_boundary(&self) -> Option<&str> {
        const BOUNDARY: &'static str = "boundary=";

        let content_type = self.content_type().unwrap();
        let start = content_type.find(BOUNDARY).unwrap() + BOUNDARY.len();
        let end = content_type[start..].find(';').map_or(content_type.len(), |end| start + end);

        Some(&content_type[start..end])
    }

    fn body(self) -> Self::Body {
        &mut (self.body.unwrap().data)[..]
    }
}
