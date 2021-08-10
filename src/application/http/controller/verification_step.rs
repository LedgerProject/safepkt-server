use crate::app;
use crate::domain;
use anyhow::Result;
use app::controller;
use domain::verification_runtime::{VerificationRuntime, VerificationStepRunner};
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;
use std::collections::HashMap;
use std::convert::Infallible;
use tracing::error;

fn change_case(step: String) -> String {
    step.replace("-", "_")
}

pub async fn get_steps(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let steps = VerificationRuntime::steps_names();
    let mut steps_names = HashMap::<String, Vec<&str>>::new();
    steps_names.insert("steps".to_string(), steps);

    controller::build_response(serde_json::to_vec(&steps_names).unwrap(), StatusCode::OK)
}

pub async fn start_running_step(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step_param = req.param("stepName").unwrap();
    let project_id = req.param("projectId").unwrap();

    let step_name = change_case(step_param.clone());
    let runtime = VerificationRuntime::new(project_id.clone(), step_name.clone()).unwrap();

    match runtime.start_running().await {
        Ok(result) => controller::ok_response(serde_json::to_vec(&result).unwrap(), StatusCode::OK),
        Err(report) => {
            error!("{}", report.to_string());

            let mut error = HashMap::<String, String>::new();
            let error_message = format!(
                "Could not run \"{}\" step for project having id \"{}\"",
                step_name, project_id
            );
            error.insert("error".to_string(), error_message);

            controller::build_response(serde_json::to_vec(&error).unwrap(), StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn get_step_report(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step_param = req.param("stepName").unwrap().clone();
    let project_id = req.param("projectId").unwrap().clone();

    let step_name = change_case(step_param);
    let runtime = VerificationRuntime::new(project_id, step_name).unwrap();

    match runtime.get_report().await {
        Ok(logs) => controller::ok_response(serde_json::to_vec(&logs).unwrap(), StatusCode::OK),
        Err(report) => {
            let mut error = HashMap::<String, String>::new();
            error.insert("error".to_string(), report.to_string());

            controller::build_response(serde_json::to_vec(&error).unwrap(), StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn get_step_progress(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let step_param = req.param("stepName").unwrap().clone();
    let project_id = req.param("projectId").unwrap().clone();

    let step_name = change_case(step_param).to_string();
    let runtime = VerificationRuntime::new(project_id, step_name).unwrap();

    match runtime.get_progress().await {
        Ok(status) => controller::ok_response(serde_json::to_vec(&status).unwrap(), StatusCode::OK),
        Err(report) => {
            let mut error = HashMap::<String, String>::new();
            error.insert("error".to_string(), report.to_string());

            controller::build_response(serde_json::to_vec(&error).unwrap(), StatusCode::BAD_REQUEST)
        }
    }
}
