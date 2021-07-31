pub mod container;

use crate::domain::verification::service::runtime::LLVMBitcodeGenerator;
use crate::domain::verification::service::runtime::VerificationRuntime;
use anyhow::Result;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;

impl VerificationRuntime<Docker> {
    pub fn new(target_hash: &'_ str) -> Result<Self, Report> {
        let docker = Docker::connect_with_socket_defaults()?;

        Ok(VerificationRuntime {
            api_client: docker,
            target_hash: String::from(target_hash),
        })
    }

    fn api_client(&self) -> &Docker {
        &self.api_client
    }

    fn target_hash(&self) -> &str {
        self.target_hash.as_str()
    }

    async fn remove_existing_container(&self) -> Result<(), Report> {
        container::remove_existing_container(self.api_client(), self.target_hash()).await?;

        Ok(())
    }

    async fn start_rvt_container(&self) -> Result<(), Report> {
        container::start_container(self.api_client(), self.target_hash()).await?;

        Ok(())
    }
}

#[async_trait]
impl LLVMBitcodeGenerator<Result<(), Report>> for VerificationRuntime<Docker> {
    async fn start_llvm_bitcode_generation(&self) -> Result<(), Report> {
        self.remove_existing_container().await?;
        self.start_rvt_container().await?;

        Ok(())
    }
}

#[async_trait]
pub trait DockerContainerAPIClient<R> {
    async fn get_container_logs(&self) -> R;
    async fn get_container_status(&self) -> R;
}

#[async_trait]
impl DockerContainerAPIClient<Result<String, Report>> for VerificationRuntime<Docker> {
    async fn get_container_logs(&self) -> Result<String, Report> {
        let logs = container::get_container_logs(self.api_client(), self.target_hash()).await?;

        Ok(logs)
    }

    async fn get_container_status(&self) -> Result<String, Report> {
        container::get_container_status(self.api_client(), self.target_hash()).await
    }
}
