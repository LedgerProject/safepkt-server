use crate::domain::verification::service::runtime as verification_runtime;
use anyhow::Result;
use hyper::header::{CONTENT_TYPE, X_CONTENT_TYPE_OPTIONS};
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::collections::HashMap;
use std::convert::Infallible;
use verification_runtime::LLVMBitcodeGenerator;
use verification_runtime::VerificationRuntime;

fn change_case(step: String) -> String {
    step.replace("-", "_")
}

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

pub async fn get_steps(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let steps = verification_runtime::VerificationRuntime::steps_names();
    let mut steps_names = HashMap::<String, Vec<&str>>::new();
    steps_names.insert("steps".to_string(), steps);

    build_response(serde_json::to_vec(&steps_names).unwrap(), StatusCode::OK)
}

pub async fn start_running_step(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step = req.param("stepName").unwrap().clone();
    let target_hash = req.param("targetHash").unwrap().as_str().clone();

    let runtime = VerificationRuntime::new(target_hash).unwrap();
    let step = change_case(step);
    let result = runtime.start_running_step(step.to_string()).await.unwrap();

    ok_response(serde_json::to_vec(&result).unwrap(), StatusCode::OK)
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

    ok_response(serde_json::to_vec(&logs).unwrap(), StatusCode::OK)
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
        Ok(status) => ok_response(serde_json::to_vec(&status).unwrap(), StatusCode::OK),
        Err(report) => {
            let mut errors = HashMap::<String, String>::new();
            errors.insert("error".to_string(), report.to_string());

            build_response(
                serde_json::to_vec(&errors).unwrap(),
                StatusCode::BAD_REQUEST,
            )
        }
    }
}
