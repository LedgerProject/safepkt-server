use crate::app;
use crate::domain;
use crate::infra;
use anyhow::Result;
use app::controller;
use domain::value_object::{Flags, Step, StepInVerificationPlan};
use domain::verification_runtime::{VerificationRuntime, VerificationStepRunner};
use hyper::{body, Body, Request, Response, StatusCode};
use infra::serializer;
use routerify::prelude::*;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str;
use tracing::error;

fn change_case(step: String) -> String {
    step.replace("-", "_")
}

fn which_step<'a>(
    steps: &HashMap<String, Step<'a>>,
    step_param: String,
    project_id: String,
) -> StepInVerificationPlan<'a> {
    let step_name = change_case(step_param.clone());
    let step = steps.get(step_name.as_str()).unwrap().clone();
    let step = StepInVerificationPlan::new(project_id.clone(), step);

    step
}

pub async fn get_steps(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let steps = VerificationRuntime::steps_names();
    let mut steps_names = HashMap::<String, Vec<&str>>::new();
    steps_names.insert("steps".to_string(), steps);

    controller::build_response(serde_json::to_vec(&steps_names).unwrap(), StatusCode::OK)
}

pub async fn start_running_step(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (head, request_body) = req.into_parts();

    let step_param = head.param("stepName").unwrap();
    let project_id = head.param("projectId").unwrap();

    let body_bytes = &body::to_bytes(request_body).await.unwrap().to_vec()[..];
    let json = str::from_utf8(&body_bytes[..].to_vec())
        .unwrap()
        .to_string();

    let steps: HashMap<String, Step>;
    let deserialized_json: Flags;

    if json.len() > 0 {
        deserialized_json = serializer::deserialize_flags(json.as_str()).unwrap();
        let flags = deserialized_json.flags();
        let additional_flags = str::from_utf8(flags).unwrap();
        steps = VerificationRuntime::build_steps(Some(additional_flags));
    } else {
        steps = VerificationRuntime::build_steps(None);
    }
    let step_in_verification_plan = which_step(&steps, step_param.clone(), project_id.clone());

    let step = step_in_verification_plan.step();
    let step = step.clone();
    let step_name = step.name().to_string().clone();

    let runtime = VerificationRuntime::new(step_in_verification_plan, steps).unwrap();

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

    let steps = VerificationRuntime::build_steps(None);
    let step = which_step(&steps, change_case(step_param), project_id.clone());
    let runtime = VerificationRuntime::new(step, steps).unwrap();

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

    let steps = VerificationRuntime::build_steps(None);
    let step = which_step(&steps, change_case(step_param), project_id.clone());
    let runtime = VerificationRuntime::new(step, steps).unwrap();

    match runtime.get_progress().await {
        Ok(status) => controller::ok_response(serde_json::to_vec(&status).unwrap(), StatusCode::OK),
        Err(report) => {
            let mut error = HashMap::<String, String>::new();
            error.insert("error".to_string(), report.to_string());

            controller::build_response(serde_json::to_vec(&error).unwrap(), StatusCode::BAD_REQUEST)
        }
    }
}
