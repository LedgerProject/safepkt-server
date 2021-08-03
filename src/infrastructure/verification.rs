pub mod runtime;

use crate::domain::verification as domain_runtime;
use crate::domain::verification::entity::step::StepInVerificationPlan;
use crate::domain::verification::service::runtime::ContainerAPIClient;
use crate::infrastructure::verification::runtime as infra_runtime;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;
use domain_runtime::entity::step::{Step, VerificationStepsCollection};
use domain_runtime::service::runtime::{VerificationRuntime, VerificationStepRunner};
use infra_runtime::docker::{container, DockerContainerAPIClient};
use std::collections::HashMap;

pub const LLVM_BITCODE_GENERATION: &str = "llvm_bitcode_generation";
pub const SYMBOLIC_EXECUTION: &str = "symbolic_execution";

impl VerificationRuntime<'_, DockerContainerAPIClient<Docker>> {
    pub fn new(project_id: String, step_name: String) -> Result<Self, Report> {
        let container_api_client = DockerContainerAPIClient::new().unwrap();

        let mut steps = HashMap::<String, Step>::new();
        steps.insert(
            LLVM_BITCODE_GENERATION.to_string(),
            Step::new(
                LLVM_BITCODE_GENERATION,
                container::llvm_bitcode_generation_cmd_provider(),
            ),
        );
        steps.insert(
            SYMBOLIC_EXECUTION.to_string(),
            Step::new(
                SYMBOLIC_EXECUTION,
                container::symbolic_execution_cmd_provider(),
            ),
        );

        let step = steps.get(step_name.as_str()).unwrap();

        Ok(VerificationRuntime {
            container_api_client,
            step_in_verification_plan: StepInVerificationPlan::new(
                String::from(project_id),
                step.clone(),
            ),
            verification_step_collection: VerificationStepsCollection::new(steps),
        })
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

        client
            .start_container(&self.step_in_verification_plan())
            .await?;

        let mut message = HashMap::<String, String>::new();

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
        names.push(SYMBOLIC_EXECUTION);

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
        let result = self.start_rvt_container(project_step).await?;

        Ok(result)
    }
}
