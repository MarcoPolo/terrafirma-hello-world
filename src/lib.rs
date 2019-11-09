#[macro_use]
extern crate http_guest;
use http_guest::{Request, Response};

fn handler(_req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    Response::builder()
        .status(200)
        .body(b"Hello World!".to_vec())
        .unwrap()
}

guest_app!(handler);
