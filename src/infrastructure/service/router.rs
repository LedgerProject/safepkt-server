use std::convert::Infallible;
use hyper::{Body, Request, Response, Method, StatusCode, body};
use base64::decode;
use std::str;
use anyhow::Result;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/source") => {
            let (_, request_body) = req.into_parts();
            let body_bytes = &body::to_bytes(request_body).await.unwrap().to_vec()[..];
            let decoded_body = decode(body_bytes).unwrap();
            let source = str::from_utf8(&decoded_body[..]).unwrap().to_string();

            *response.body_mut() = Body::from(source);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}
