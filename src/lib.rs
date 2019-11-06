extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate http_guest;

use http_guest::{Request, Response};

// Hello World
pub fn server(_req: &Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error>> {
    Ok(Response::builder()
        .status(200)
        .body(b"Hello World!".to_vec())
        .unwrap())
}

/// Wrapper for the server that turns the error case into a 500 response showing
/// the error:
fn server_(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    match server(req) {
        Ok(resp) => resp,
        Err(_) => {
            // let body = format!("Demo Error: {:?}", e);
            let body = "Demo Error";
            Response::builder()
                .status(500)
                .body(body.as_bytes().to_owned())
                .unwrap()
        }
    }
}

guest_app!(server_);
