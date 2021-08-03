use crate::domain::verification::service::runtime as verification_runtime;
use anyhow::Result;
use hyper::header::{CONTENT_TYPE, X_CONTENT_TYPE_OPTIONS};
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::collections::HashMap;
use std::convert::Infallible;
use verification_runtime::{VerificationRuntime, VerificationStepRunner};

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
    let step_param = req.param("stepName").unwrap().clone();
    let project_id = req.param("projectId").unwrap().clone();

    let step_name = change_case(step_param);
    let runtime = VerificationRuntime::new(project_id, step_name).unwrap();
    let result = runtime.start_running().await.unwrap();

    ok_response(serde_json::to_vec(&result).unwrap(), StatusCode::OK)
}

pub async fn get_step_report(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step_param = req.param("stepName").unwrap().clone();
    let project_id = req.param("projectId").unwrap().clone();

    let step_name = change_case(step_param);
    let runtime = VerificationRuntime::new(project_id, step_name).unwrap();

    match runtime.get_report().await {
        Ok(logs) => ok_response(serde_json::to_vec(&logs).unwrap(), StatusCode::OK),
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

pub async fn get_step_progress(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step_param = req.param("stepName").unwrap().clone();
    let project_id = req.param("projectId").unwrap().clone();

    let step_name = change_case(step_param).to_string();
    let runtime = VerificationRuntime::new(project_id, step_name).unwrap();

    match runtime.get_progress().await {
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
