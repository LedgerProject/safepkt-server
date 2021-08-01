pub mod container;

use anyhow::Result;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;

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
    async fn tail_container_logs(&self, name: &str) -> R;
    async fn inspect_container_status(&self, name: &str) -> R;
}

#[async_trait]
impl DockerContainerAPIClient<Result<String, Report>> for ContainerAPIClient<Docker> {
    async fn tail_container_logs(&self, name: &str) -> Result<String, Report> {
        let logs = container::tail_container_logs(&self, name).await?;

        Ok(logs)
    }

    async fn inspect_container_status(&self, name: &str) -> Result<String, Report> {
        container::inspect_container_status(&self, name).await
    }
}
