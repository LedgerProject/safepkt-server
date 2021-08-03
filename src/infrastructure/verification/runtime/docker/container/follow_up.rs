use crate::infrastructure::verification::runtime::docker::ContainerAPIClient;
use anyhow::Result;
use bollard::container::{InspectContainerOptions, ListContainersOptions, LogOutput, LogsOptions};
use bollard::models::*;
use bollard::Docker;
use color_eyre::{eyre::eyre, Report};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::default::Default;
use std::str;
use tracing::{debug, info};

pub async fn tail_container_logs(
    container_api_client: &ContainerAPIClient<Docker>,
    container_name: &str,
) -> Result<String, Report> {
    let mut logs_stream = container_api_client.client().logs(
        container_name,
        Some(LogsOptions::<String> {
            stdout: true,
            stderr: true,
            ..Default::default()
        }),
    );

    debug!("About to tail logs for container \"{}\"", container_name);
    let mut logs: Vec<String> = vec![String::from("")];

    while let Some(Ok(log)) = logs_stream.next().await {
        match log {
            LogOutput::StdOut { message } => {
                let message = str::from_utf8(&*message).unwrap();
                info!("[STDOUT] {}", message);
                logs.push(String::from(message))
            }
            LogOutput::StdErr { message } => {
                let message = str::from_utf8(&*message).unwrap();
                info!("[STDERR] {}", message);
                logs.push(String::from(message))
            }
            LogOutput::Console { message } => {
                let message = str::from_utf8(&*message).unwrap();
                info!("[CONSOLE] {}", message);
                logs.push(String::from(message))
            }
            _ => {}
        }
    }

    Ok(logs.join(""))
}

async fn get_status(
    container_api_client: &ContainerAPIClient<Docker>,
    container_summary: &ContainerSummaryInner,
) -> Result<String, Report> {
    let container_inspect_response = container_api_client
        .client()
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

pub async fn inspect_container_status(
    container_api_client: &ContainerAPIClient<Docker>,
    container_name: &str,
) -> Result<String, Report> {
    let mut list_container_filters = HashMap::new();
    list_container_filters.insert("name", vec![container_name]);

    let containers = container_api_client
        .client()
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters: list_container_filters,
            ..Default::default()
        }))
        .await?;

    match containers.first() {
        Some(container_summary_inner) => {
            let status = get_status(container_api_client, container_summary_inner).await?;

            Ok(status)
        }
        _ => Err(eyre!(format!(
            "No status available for container \"{}\"",
            container_name
        ))),
    }
}
