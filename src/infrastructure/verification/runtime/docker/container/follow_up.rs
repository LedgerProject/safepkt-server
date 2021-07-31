use anyhow::Result;
use bollard::container::{InspectContainerOptions, ListContainersOptions, LogOutput, LogsOptions};
use bollard::models::*;
use bollard::Docker;
use color_eyre::{eyre::eyre, Report};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::default::Default;
use std::str;
use tracing::info;

pub async fn get_container_logs(api_client: &Docker, target_hash: &str) -> Result<String, Report> {
    let mut logs_stream = api_client.logs(
        target_hash,
        Some(LogsOptions::<String> {
            stdout: true,
            ..Default::default()
        }),
    );

    let mut logs: Vec<String> = vec![String::from("")];

    while let Some(Ok(log)) = logs_stream.next().await {
        if let LogOutput::StdOut { message } = log {
            let message = str::from_utf8(&*message).unwrap();
            info!("{}", message);
            logs.push(String::from(message))
        }
    }

    Ok(logs.join(""))
}

async fn get_status<'a>(
    api_client: &'a Docker,
    container_summary: &'a ContainerSummaryInner,
) -> Result<String, Report> {
    let container_inspect_response = api_client
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

pub async fn get_container_status(
    api_client: &Docker,
    target_hash: &str,
) -> Result<String, Report> {
    let mut list_container_filters = HashMap::new();
    list_container_filters.insert("name", vec![target_hash]);

    let containers = api_client
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters: list_container_filters,
            ..Default::default()
        }))
        .await?;

    match containers.first() {
        Some(container_summary_inner) => {
            let status = get_status(api_client, container_summary_inner).await?;

            Ok(status)
        }
        _ => Err(eyre!(format!(
            "No status available for container \"{}\"",
            target_hash
        ))),
    }
}
