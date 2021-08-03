pub mod container;

use anyhow::Result;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;
use std::collections::HashMap;

pub struct ContainerAPIClient<C> {
    client: C,
}

impl ContainerAPIClient<Docker> {
    pub fn new() -> Result<Self, Report> {
        let docker = Docker::connect_with_socket_defaults()?;

        Ok(ContainerAPIClient { client: docker })
    }

    pub fn client(&self) -> &Docker {
        &self.client
    }
}

#[async_trait]
pub trait DockerContainerAPIClient<R> {
    async fn tail_container_logs(&self, container_name: &str) -> R;
    async fn inspect_container_status(&self, container_name: &str) -> R;
}

#[async_trait]
impl DockerContainerAPIClient<Result<HashMap<String, String>, Report>>
    for ContainerAPIClient<Docker>
{
    async fn tail_container_logs(
        &self,
        container_name: &str,
    ) -> Result<HashMap<String, String>, Report> {
        let logs = container::tail_container_logs(&self, container_name).await?;

        Ok(logs)
    }

    async fn inspect_container_status(
        &self,
        container_name: &str,
    ) -> Result<HashMap<String, String>, Report> {
        container::inspect_container_status(&self, container_name).await
    }
}
