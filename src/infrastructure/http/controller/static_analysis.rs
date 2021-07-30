use std::convert::Infallible;
use hyper::{Body, Request, Response};
use routerify::prelude::*;

pub async fn new_static_analysis(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let source_hash = req.param("sourceHash").unwrap().as_str().clone();

    Ok(Response::new(Body::from(String::from(source_hash))))
}