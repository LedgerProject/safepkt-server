use crate::application::project::scaffold::scaffold_project;
use crate::domain::verification::service::runtime as verification_runtime;
use crate::infrastructure::verification::runtime::docker::DockerContainerAPIClient;
use anyhow::Result;
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::convert::Infallible;
use verification_runtime::LLVMBitcodeGenerator;
use verification_runtime::VerificationRuntime;

pub async fn start_llvm_bitcode_generation(
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let target_hash = req.param("targetHash").unwrap().as_str().clone();
    scaffold_project(target_hash).await.unwrap();

    let runtime = VerificationRuntime::new(target_hash).unwrap();
    runtime.start_llvm_bitcode_generation().await.unwrap();

    Ok(Response::new(Body::from(String::from(target_hash))))
}

pub async fn get_logs(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let client = VerificationRuntime::new(target_hash).unwrap();
    let logs = client
        .container_api_client()
        .tail_container_logs(target_hash)
        .await
        .unwrap();

    Ok(Response::new(Body::from(logs)))
}

pub async fn get_status(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let client = VerificationRuntime::new(target_hash).unwrap();

    match client
        .container_api_client()
        .inspect_container_status(target_hash)
        .await
    {
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
