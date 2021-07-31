use crate::infrastructure::docker::client::Client;
use anyhow::Result;
use hyper::{Body, Request, Response};
use routerify::prelude::*;
use std::convert::Infallible;

pub async fn container_logs(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let source_hash = req.param("sourceHash").unwrap().as_str().clone();

    let client = Client::new(source_hash).unwrap();
    let logs = client.get_container_logs().await.unwrap();

    Ok(Response::new(Body::from(logs)))
}
