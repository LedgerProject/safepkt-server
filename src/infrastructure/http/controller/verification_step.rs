use crate::domain::verification::service::runtime as verification_runtime;
use anyhow::Result;
use hyper::header::CONTENT_TYPE;
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::convert::Infallible;
use verification_runtime::LLVMBitcodeGenerator;
use verification_runtime::VerificationRuntime;

fn change_case(step: String) -> String {
    step.replace("-", "_")
}

pub async fn get_steps(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = verification_runtime::VerificationRuntime::steps_names();
    let steps = serde_json::to_vec(&body).unwrap();

    Ok(Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(steps))
        .unwrap())
}

pub async fn start_running_step(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step = req.param("stepName").unwrap().clone();
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let runtime = VerificationRuntime::new(target_hash).unwrap();
    let step = change_case(step);
    runtime.start_running_step(step.to_string()).await.unwrap();

    Ok(Response::new(Body::from(String::from(target_hash))))
}

pub async fn tail_logs(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step = req.param("stepName").unwrap().clone();
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let step = change_case(step);
    let logs = VerificationRuntime::new(target_hash)
        .unwrap()
        .tail_logs_for_step(step.to_string())
        .await
        .unwrap();

    Ok(Response::new(Body::from(logs)))
}

pub async fn get_progress(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step = req.param("stepName").unwrap().clone();
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let step = change_case(step);
    match VerificationRuntime::new(target_hash)
        .unwrap()
        .get_progress_for_step(step.to_string())
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
