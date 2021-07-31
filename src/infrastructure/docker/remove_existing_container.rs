use anyhow::Result;
use bollard::container::{ListContainersOptions, RemoveContainerOptions};
use bollard::Docker;
use color_eyre::Report;
use std::collections::HashMap;

async fn container_exists(docker: &Docker, name: &str) -> Result<bool, Report> {
    let mut filters = HashMap::new();
    filters.insert("name", vec![name]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = docker.list_containers(options).await?;

    Ok(!containers.is_empty())
}

pub async fn remove_existing_container(docker: &Docker, source_hash: &str) -> Result<(), Report> {
    let existing_container = container_exists(&docker, source_hash).await.unwrap();

    if existing_container {
        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });

        docker.remove_container(source_hash, options).await.unwrap();
    }

    Ok(())
}
