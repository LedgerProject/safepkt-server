use crate::infrastructure::docker::remove_existing_container::remove_existing_container;
use crate::infrastructure::docker::start_rvt_container::start_rvt_container;
use crate::infrastructure::docker::tail_container_logs::get_container_logs;
use anyhow::Result;
use bollard::Docker;
use color_eyre::Report;

pub struct Client {
    docker: Docker,
    source_hash: String,
}

impl Client {
    pub fn new(source_hash: &'_ str) -> Result<Self, Report> {
        let docker = Docker::connect_with_socket_defaults()?;

        Ok(Client {
            docker,
            source_hash: String::from(source_hash),
        })
    }

    pub fn docker(&self) -> &Docker {
        &self.docker
    }

    pub fn source_hash(&self) -> &str {
        self.source_hash.as_str()
    }

    async fn remove_existing_container(&self) -> Result<(), Report> {
        remove_existing_container(self.docker(), self.source_hash()).await?;

        Ok(())
    }

    async fn start_rvt_container(&self) -> Result<(), Report> {
        start_rvt_container(self.docker(), self.source_hash()).await?;

        Ok(())
    }

    pub async fn start_static_analysis(&self) -> Result<(), Report> {
        self.remove_existing_container().await?;
        self.start_rvt_container().await?;

        Ok(())
    }

    pub async fn get_container_logs(&self) -> Result<String, Report> {
        let logs = get_container_logs(self.docker(), self.source_hash()).await?;

        Ok(logs)
    }
}
