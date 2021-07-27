use std::convert::Infallible;
use hyper::{Body, Request, Response, Method, StatusCode};

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/source") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}
