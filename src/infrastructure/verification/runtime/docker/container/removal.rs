use crate::infrastructure as infra;
use anyhow::Result;
use bollard::container::RemoveContainerOptions;
use bollard::Docker;
use color_eyre::Report;
use infra::verification::runtime::docker::{container::container_exists, DockerContainerAPIClient};

pub async fn remove_existing_container(
    container_api_client: &DockerContainerAPIClient<Docker>,
    container_name: String,
) -> Result<(), Report> {
    let existing_container = container_exists(container_api_client, container_name.as_str())
        .await
        .unwrap();

    if existing_container {
        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });

        container_api_client
            .client()
            .remove_container(container_name.as_str(), options)
            .await
            .unwrap();
    }

    Ok(())
}
