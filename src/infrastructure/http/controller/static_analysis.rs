use crate::application::project::scaffold::scaffold_project;
use crate::infrastructure::docker::client::start_static_analysis;
use anyhow::Result;
use hyper::{Body, Request, Response};
use routerify::prelude::*;
use std::convert::Infallible;

pub async fn new_static_analysis(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let source_hash = req.param("sourceHash").unwrap().as_str().clone();
    scaffold_project(source_hash).await.unwrap();
    start_static_analysis(source_hash).await.unwrap();

    Ok(Response::new(Body::from(String::from(source_hash))))
}
