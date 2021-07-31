use crate::infrastructure::docker::remove_existing_container::remove_existing_container;
use crate::infrastructure::docker::start_rvt_container::start_rvt_container;
use anyhow::Result;
use bollard::Docker;
use color_eyre::Report;

pub async fn start_static_analysis(source_hash: &str) -> Result<(), Report> {
    let docker = &Docker::connect_with_socket_defaults()?;

    remove_existing_container(docker, source_hash).await?;
    start_rvt_container(docker, source_hash).await?;

    Ok(())
}
