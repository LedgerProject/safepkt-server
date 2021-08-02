use crate::domain::verification::service::runtime as verification_runtime;
use crate::infrastructure::verification::SYMBOLIC_EXECUTION;
use anyhow::Result;
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::convert::Infallible;
use verification_runtime::LLVMBitcodeGenerator;
use verification_runtime::VerificationRuntime;

pub async fn start_running_step(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let runtime = VerificationRuntime::new(target_hash).unwrap();
    runtime
        .start_running_step(SYMBOLIC_EXECUTION.to_string())
        .await
        .unwrap();

    Ok(Response::new(Body::from(String::from(target_hash))))
}

pub async fn get_logs(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let logs = VerificationRuntime::new(target_hash)
        .unwrap()
        .tail_logs_for_step(SYMBOLIC_EXECUTION.to_string())
        .await
        .unwrap();

    Ok(Response::new(Body::from(logs)))
}

pub async fn get_status(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    match VerificationRuntime::new(target_hash)
        .unwrap()
        .get_progress_for_step(SYMBOLIC_EXECUTION.to_string())
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
