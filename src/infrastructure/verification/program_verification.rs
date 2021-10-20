use crate::domain;
use crate::infra::display;

use async_trait::async_trait;
use color_eyre::{eyre::eyre, Report};
use domain::program_verification::*;
use domain::value_object::{Step, StepInVerificationPlan};
use domain::verification_runtime::{VerificationRuntime, VerificationStepRunner};
use std::collections::HashMap;

pub fn change_case(step: String) -> String {
    step.replace("-", "_")
}

pub fn which_step<'a>(
    steps: &HashMap<String, Step<'a>>,
    step_param: String,
    project_id: String,
) -> StepInVerificationPlan<'a> {
    let step_name = change_case(step_param.clone());
    let step = steps.get(step_name.as_str()).unwrap().clone();
    let step = StepInVerificationPlan::new(project_id.clone(), step);

    step
}

impl VerificationTarget<'_> {
    pub fn new<'a>(step: &'a str, project_id: &'a str) -> VerificationTarget<'a> {
        VerificationTarget { step, project_id }
    }
}

#[async_trait]
impl<'a> ProgramVerification for SmartContractVerification<'a> {
    type A = VerificationTarget<'a>;
    type R = Result<HashMap<String, String>, Report>;

    fn new(target: Self::A) -> Self {
        SmartContractVerification { target }
    }

    async fn run_step(&self) -> Self::R {
        let steps: HashMap<String, Step>;

        let target_step = self.target.step.to_string();
        let project_id = self.target.project_id.to_string();

        steps = VerificationRuntime::build_steps(None);
        let step_in_verification_plan = which_step(&steps, target_step, project_id.clone());

        let step = step_in_verification_plan.step();
        let step = step.clone();
        let step_name = step.name().to_string().clone();

        let runtime = VerificationRuntime::new(step_in_verification_plan, steps).unwrap();

        match runtime.start_running().await {
            Ok(result) => Ok(result),
            Err(report) => {
                display::output::eprint("{}", vec![report.to_string().as_str()], None);

                let error_message = format!(
                    "Could not run \"{}\" step for project having id \"{}\"",
                    step_name, project_id
                );

                Err(eyre!(error_message))
            }
        }
    }

    async fn step_report(&self) -> Self::R {
        let target_step = self.target.step.to_string();
        let project_id = self.target.project_id.to_string();

        let steps = VerificationRuntime::build_steps(None);
        let step = which_step(&steps, change_case(target_step), project_id.clone());
        let runtime = VerificationRuntime::new(step, steps).unwrap();

        match runtime.get_report().await {
            Ok(logs) => Ok(logs),
            Err(report) => {
                let err = report.to_string();
                display::output::eprint("{}", vec![err.clone().as_str()], None);
                Err(eyre!(err))
            }
        }
    }

    async fn step_progress(&self) -> Self::R {
        let target_step = self.target.step.to_string();
        let project_id = self.target.project_id.to_string();

        let steps = VerificationRuntime::build_steps(None);
        let step = which_step(&steps, change_case(target_step), project_id.clone());
        let runtime = VerificationRuntime::new(step, steps).unwrap();

        match runtime.get_progress().await {
            Ok(status) => Ok(status),
            Err(report) => {
                let err = report.to_string();
                display::output::eprint("{}", vec![err.clone().as_str()], None);
                Err(eyre!(err))
            }
        }
    }
}
