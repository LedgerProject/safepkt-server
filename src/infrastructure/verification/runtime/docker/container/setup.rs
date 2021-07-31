use anyhow::Result;
use bollard::container::{ListContainersOptions, RemoveContainerOptions};
use bollard::Docker;
use color_eyre::Report;
use std::collections::HashMap;

async fn container_exists(api_client: &Docker, name: &str) -> Result<bool, Report> {
    let mut filters = HashMap::new();
    filters.insert("name", vec![name]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = api_client.list_containers(options).await?;

    Ok(!containers.is_empty())
}

pub async fn remove_existing_container(
    api_client: &Docker,
    target_hash: &str,
) -> Result<(), Report> {
    let existing_container = container_exists(&api_client, target_hash).await.unwrap();

    if existing_container {
        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });

        api_client
            .remove_container(target_hash, options)
            .await
            .unwrap();
    }

    Ok(())
}
