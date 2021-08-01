pub mod container;

use crate::domain::verification::service::runtime as verification_runtime;
use anyhow::Result;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;
use verification_runtime::LLVMBitcodeGenerator;
use verification_runtime::VerificationRuntime;

impl VerificationRuntime<ContainerAPIClient<Docker>> {
    pub fn new(target_hash: &'_ str) -> Result<Self, Report> {
        let container_api_client = ContainerAPIClient::new().unwrap();

        Ok(VerificationRuntime {
            container_api_client,
            target_hash: String::from(target_hash),
        })
    }

    pub fn container_api_client(&self) -> &ContainerAPIClient<Docker> {
        &self.container_api_client
    }

    pub fn target_hash(&self) -> &str {
        self.target_hash.as_str()
    }

    async fn remove_existing_container(&self) -> Result<(), Report> {
        container::remove_existing_container(self.container_api_client(), self.target_hash())
            .await?;

        Ok(())
    }

    async fn start_rvt_container(&self) -> Result<(), Report> {
        container::start_container(self.container_api_client(), self.target_hash()).await?;

        Ok(())
    }
}

#[async_trait]
impl LLVMBitcodeGenerator<Result<(), Report>> for VerificationRuntime<ContainerAPIClient<Docker>> {
    async fn start_llvm_bitcode_generation(&self) -> Result<(), Report> {
        self.remove_existing_container().await?;
        self.start_rvt_container().await?;

        Ok(())
    }
}

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
