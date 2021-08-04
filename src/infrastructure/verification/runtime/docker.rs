pub mod container;

use crate::domain::value_object::*;
use crate::domain::verification_runtime::*;
use anyhow::Result;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;
use std::collections::HashMap;

pub struct DockerContainerAPIClient<C> {
    client: C,
}

impl DockerContainerAPIClient<Docker> {
    pub fn new() -> Result<Self, Report> {
        let docker = Docker::connect_with_socket_defaults()?;

        Ok(DockerContainerAPIClient { client: docker })
    }

    pub fn client(&self) -> &Docker {
        &self.client
    }

    pub fn format_container_name_for_step_in_verification_plan(
        &self,
        project_step: &StepInVerificationPlan,
    ) -> String {
        format!(
            "{}-{}",
            project_step.step().name().clone(),
            project_step.project_id().clone()
        )
    }
}

#[async_trait]
impl ContainerAPIClient for DockerContainerAPIClient<Docker> {
    type R = Result<HashMap<String, String>, Report>;
    type P = Result<(), Report>;

    async fn inspect_container_status(&self, project_step: &StepInVerificationPlan) -> Self::R {
        let container_name = self.format_container_name_for_step_in_verification_plan(project_step);
        container::inspect_container_status(&self, container_name.as_str()).await
    }

    async fn remove_existing_container(&self, project_step: &StepInVerificationPlan) -> Self::P {
        let container_name = self.format_container_name_for_step_in_verification_plan(project_step);
        container::remove_existing_container(&self, container_name).await
    }

    async fn start_container(&self, project_step: &StepInVerificationPlan) -> Self::P {
        let container_name = self.format_container_name_for_step_in_verification_plan(project_step);
        container::start_container(&self, container_name, project_step).await
    }

    async fn tail_container_logs(&self, project_step: &StepInVerificationPlan) -> Self::R {
        let container_name = self.format_container_name_for_step_in_verification_plan(project_step);
        container::tail_container_logs(&self, container_name.as_str()).await
    }
}
