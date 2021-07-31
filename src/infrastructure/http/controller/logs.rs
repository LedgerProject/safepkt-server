use crate::infrastructure::docker::client::Client;
use anyhow::Result;
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::convert::Infallible;

pub async fn get_container_logs(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let source_hash = req.param("sourceHash").unwrap().as_str().clone();

    let client = Client::new(source_hash).unwrap();
    let logs = client.get_container_logs().await.unwrap();

    Ok(Response::new(Body::from(logs)))
}

pub async fn get_static_analysis_status(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let source_hash = req.param("sourceHash").unwrap().as_str().clone();

    let client = Client::new(source_hash).unwrap();

    match client.get_static_analysis_status().await {
        Ok(status) => Ok(Response::new(Body::from(status))),
        Err(report) => {
            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(report.to_string()))
                .unwrap();

            Ok(response)
        }
    }
}
