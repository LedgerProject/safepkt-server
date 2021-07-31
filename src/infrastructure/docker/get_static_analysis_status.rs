use anyhow::Result;
use bollard::container::{InspectContainerOptions, ListContainersOptions};
use bollard::models::*;
use bollard::Docker;
use color_eyre::{eyre::eyre, Report};
use std::collections::HashMap;

async fn get_status<'a>(
    docker: &'a Docker,
    container_summary: &'a ContainerSummaryInner,
) -> Result<String, Report> {
    let container_inspect_response = docker
        .inspect_container(
            container_summary.id.as_ref().unwrap(),
            None::<InspectContainerOptions>,
        )
        .await
        .unwrap();

    if let Some(state) = container_inspect_response.state {
        if let Some(status) = state.status {
            return Ok(status.to_string());
        }
    }

    unreachable!()
}

pub async fn get_static_analysis_status(
    docker: &Docker,
    source_hash: &str,
) -> Result<String, Report> {
    let mut list_container_filters = HashMap::new();
    list_container_filters.insert("name", vec![source_hash]);

    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters: list_container_filters,
            ..Default::default()
        }))
        .await?;

    match containers.first() {
        Some(container_summary_inner) => {
            let status = get_status(docker, container_summary_inner).await?;

            Ok(status)
        }
        _ => Err(eyre!(format!(
            "No status available for container \"{}\"",
            source_hash
        ))),
    }
}
