pub mod runtime;

use crate::domain::verification::service::runtime as verification_runtime;
use async_trait::async_trait;
use bollard::Docker;
use color_eyre::Report;
use runtime::docker::container;
use runtime::docker::ContainerAPIClient;
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
