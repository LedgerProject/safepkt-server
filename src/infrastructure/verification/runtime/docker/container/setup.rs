use crate::infrastructure::verification::runtime::docker::ContainerAPIClient;
use anyhow::Result;
use bollard::container::{ListContainersOptions, RemoveContainerOptions};
use bollard::Docker;
use color_eyre::Report;
use std::collections::HashMap;

async fn container_exists(
    container_api_client: &ContainerAPIClient<Docker>,
    container_name: &str,
) -> Result<bool, Report> {
    let mut filters = HashMap::new();
    filters.insert("name", vec![container_name]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = container_api_client
        .client()
        .list_containers(options)
        .await?;

    Ok(!containers.is_empty())
}

pub async fn remove_existing_container(
    container_api_client: &ContainerAPIClient<Docker>,
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
