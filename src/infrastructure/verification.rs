pub mod runtime;

use crate::domain::verification as domain_runtime;
use crate::infrastructure::verification::runtime as infra_runtime;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::{eyre::eyre, Report};
use domain_runtime::entity::verification_steps_collection::{Step, VerificationStepsCollection};
use domain_runtime::service::runtime::{LLVMBitcodeGenerator, VerificationRuntime};
use infra_runtime::docker::{container, ContainerAPIClient, DockerContainerAPIClient};
use std::collections::HashMap;

pub const LLVM_BITCODE_GENERATION: &str = "llvm_bitcode_generation";
pub const SYMBOLIC_EXECUTION: &str = "symbolic_execution";

impl VerificationRuntime<'_, ContainerAPIClient<Docker>> {
    pub fn new(target_hash: &'_ str) -> Result<Self, Report> {
        let container_api_client = ContainerAPIClient::new().unwrap();

        let mut steps = HashMap::<&str, Step>::new();
        steps.insert(
            LLVM_BITCODE_GENERATION,
            Step::new(
                LLVM_BITCODE_GENERATION.to_string(),
                container::llvm_bitcode_generation_cmd_provider(),
            ),
        );
        steps.insert(
            SYMBOLIC_EXECUTION,
            Step::new(
                SYMBOLIC_EXECUTION.to_string(),
                container::symbolic_execution_cmd_provider(),
            ),
        );

        Ok(VerificationRuntime {
            container_api_client,
            target_hash: String::from(target_hash),
            verification_step_collection: VerificationStepsCollection::new(steps),
        })
    }

    pub fn container_api_client(&self) -> &ContainerAPIClient<Docker> {
        &self.container_api_client
    }

    pub fn target_hash(&self) -> &str {
        self.target_hash.as_str()
    }

    pub fn verification_step_collection(&self) -> &VerificationStepsCollection {
        &self.verification_step_collection
    }

    pub fn get_container_name_for(&self, step_name: String) -> String {
        format!("{}-{}", &step_name, self.target_hash())
    }

    async fn remove_existing_container(&self, step: &Step) -> Result<(), Report> {
        container::remove_existing_container(
            self.container_api_client(),
            self.get_container_name_for(step.name().clone()),
        )
        .await?;

        Ok(())
    }

    async fn start_rvt_container(&self, step: &Step) -> Result<(), Report> {
        container::start_container(
            self.container_api_client(),
            self.get_container_name_for(step.name().clone()),
            step,
            String::from(self.target_hash()),
        )
        .await?;

        Ok(())
    }
}

#[async_trait]
impl LLVMBitcodeGenerator<Result<(), Report>, Result<String, Report>>
    for VerificationRuntime<'_, ContainerAPIClient<Docker>>
{
    async fn start_running_step(&self, step_name: String) -> Result<(), Report> {
        let step = self.verification_step_collection.step(&step_name);

        self.remove_existing_container(step).await?;
        self.start_rvt_container(step).await?;

        Ok(())
    }

    async fn tail_logs_for_step(&self, step_name: String) -> Result<String, Report> {
        let logs = self
            .container_api_client()
            .tail_container_logs(self.get_container_name_for(step_name).as_str())
            .await
            .unwrap();

        Ok(logs)
    }

    async fn get_progress_for_step(&self, step_name: String) -> Result<String, Report> {
        match self
            .container_api_client()
            .inspect_container_status(self.get_container_name_for(step_name).as_str())
            .await
        {
            Ok(status) => Ok(status),
            Err(report) => Err(eyre!(report.to_string())),
        }
    }
}
