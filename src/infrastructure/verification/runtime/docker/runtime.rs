use anyhow::Result;
use bollard::Docker;
use color_eyre::Report;

use super::container;
use crate::domain::verification::service::runtime::VerificationRuntime;

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

    pub async fn start_static_analysis(&self) -> Result<(), Report> {
        self.remove_existing_container().await?;
        self.start_rvt_container().await?;

        Ok(())
    }

    pub async fn get_container_logs(&self) -> Result<String, Report> {
        let logs = container::get_container_logs(self.api_client(), self.target_hash()).await?;

        Ok(logs)
    }

    pub async fn get_static_analysis_status(&self) -> Result<String, Report> {
        container::get_container_status(self.api_client(), self.target_hash()).await
    }
}
