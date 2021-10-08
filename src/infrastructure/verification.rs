pub mod runtime;

use crate::domain::value_object::*;
use crate::domain::verification_runtime::*;
use crate::infra;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;
use infra::scaffold;
use infra::verification_runtime::docker::{container, DockerContainerAPIClient};
use std::collections::HashMap;

pub const LLVM_BITCODE_GENERATION: &str = "llvm_bitcode_generation";
pub const SYMBOLIC_EXECUTION: &str = "symbolic_execution";
pub const PROGRAM_VERIFICATION: &str = "program_verification";
pub const SOURCE_RESTORATION: &str = "source_restoration";

impl<'a> VerificationRuntime<'a, DockerContainerAPIClient<Docker>> {
    pub fn new(
        step: StepInVerificationPlan<'a>,
        steps: HashMap<String, Step<'a>>,
    ) -> Result<Self, Report> {
        let container_api_client = DockerContainerAPIClient::new().unwrap();
        let runtime = VerificationRuntime {
            container_api_client,
            step_in_verification_plan: step,
            verification_step_collection: VerificationStepsCollection::new(steps),
        };

        Ok(runtime)
    }

    pub fn build_steps(flags: Option<&str>) -> HashMap<String, Step> {
        let mut steps = HashMap::<String, Step>::new();

        steps.insert(
            LLVM_BITCODE_GENERATION.to_string(),
            Step::new(
                LLVM_BITCODE_GENERATION,
                container::llvm_bitcode_generation_cmd_provider(),
                None,
            ),
        );

        steps.insert(
            SYMBOLIC_EXECUTION.to_string(),
            Step::new(
                SYMBOLIC_EXECUTION,
                container::symbolic_execution_cmd_provider(),
                flags,
            ),
        );

        steps.insert(
            PROGRAM_VERIFICATION.to_string(),
            Step::new(
                PROGRAM_VERIFICATION,
                container::symbolic_execution_cmd_provider(),
                flags,
            ),
        );

        steps.insert(
            SOURCE_RESTORATION.to_string(),
            Step::new(
                SOURCE_RESTORATION,
                container::source_code_restoration_cmd_provider(),
                None,
            ),
        );

        steps
    }

    pub fn container_api_client(&self) -> &DockerContainerAPIClient<Docker> {
        &self.container_api_client
    }

    pub fn project_id(&self) -> &str {
        self.step_in_verification_plan.project_id().as_str()
    }

    pub fn project_step(&self) -> &Step {
        &self.step_in_verification_plan.step()
    }

    pub fn step_in_verification_plan(&self) -> &StepInVerificationPlan {
        &self.step_in_verification_plan
    }

    pub fn verification_step_collection(&self) -> &VerificationStepsCollection {
        &self.verification_step_collection
    }

    async fn remove_existing_container(&self) -> Result<(), Report> {
        let client = self.container_api_client();
        client
            .remove_existing_container(self.step_in_verification_plan())
            .await?;

        Ok(())
    }

    async fn start_rvt_container(
        &self,
        project_step: &StepInVerificationPlan<'_>,
    ) -> Result<HashMap<String, String>, Report> {
        let client = self.container_api_client();

        let mut message = HashMap::<String, String>::new();

        match client
            .start_container(&self.step_in_verification_plan())
            .await
        {
            Ok(_) => {}
            Err(report) => return Err(report),
        }

        message.insert(
            "container_name".to_string(),
            client
                .format_container_name_for_step_in_verification_plan(project_step)
                .clone(),
        );
        message.insert(
            "message".to_string(),
            String::from("Rust verification tools container started successfully."),
        );

        Ok(message)
    }
}

#[async_trait]
impl VerificationStepRunner<Result<HashMap<String, String>, Report>>
    for VerificationRuntime<'_, DockerContainerAPIClient<Docker>>
{
    fn steps_names() -> Vec<&'static str> {
        let mut names = Vec::<&str>::new();
        names.push(LLVM_BITCODE_GENERATION);
        names.push(LLVM_BITCODE_GENERATION);
        names.push(SYMBOLIC_EXECUTION);
        names.push(SOURCE_RESTORATION);

        names
    }

    async fn get_progress(&self) -> Result<HashMap<String, String>, Report> {
        self.container_api_client()
            .inspect_container_status(self.step_in_verification_plan())
            .await
    }

    async fn get_report(&self) -> Result<HashMap<String, String>, Report> {
        self.container_api_client()
            .tail_container_logs(self.step_in_verification_plan())
            .await
    }

    async fn start_running(&self) -> Result<HashMap<String, String>, Report> {
        self.remove_existing_container().await?;

        let project_step = self.step_in_verification_plan();

        // if scaffold::scaffold_project(project_step.project_id()).is_ok() {
        //     let result = self.start_rvt_container(project_step).await?;
        //
        //     return Ok(result);
        // }

        if scaffold::scaffold_library(project_step.project_id()).is_ok() {
            let result = self.start_rvt_container(project_step).await?;

            return Ok(result);
        }

        unreachable!()
    }
}
