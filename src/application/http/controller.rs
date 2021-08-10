use hyper::header::{CONTENT_TYPE, X_CONTENT_TYPE_OPTIONS};
use hyper::{Body, Response, StatusCode};
use std::convert::Infallible;

pub mod source;
pub mod verification_step;

fn build_response(body: Vec<u8>, status_code: StatusCode) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(X_CONTENT_TYPE_OPTIONS, "nosniff")
        .status(status_code)
        .body(Body::from(body))
        .unwrap())
}

fn ok_response(body: Vec<u8>, status_code: StatusCode) -> Result<Response<Body>, Infallible> {
    build_response(body, status_code)
}
