use crate::infrastructure as infra;
use anyhow::Result;
use bollard::container::{InspectContainerOptions, ListContainersOptions, LogOutput, LogsOptions};
use bollard::models::*;
use bollard::Docker;
use color_eyre::{eyre::eyre, Report};
use futures::stream::StreamExt;
use infra::display::output;
use infra::verification::runtime::docker::DockerContainerAPIClient;
use regex::Regex;
use std::collections::HashMap;
use std::default::Default;
use std::str;
use tracing::debug;

pub async fn container_exists(
    container_api_client: &DockerContainerAPIClient<Docker>,
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

pub async fn tail_container_logs<'a>(
    container_api_client: &DockerContainerAPIClient<Docker>,
    container_name: &str,
) -> Result<HashMap<String, String>, Report> {
    if !container_exists(container_api_client, container_name)
        .await
        .unwrap()
    {
        return Err(eyre!(
            "There is no container having name \"{}\"",
            container_name
        ));
    }

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
                let message = str::from_utf8(&*message)?;
                let re = Regex::new(r"^STDERR:.+")?;

                if re.is_match(message) {
                    output::print("{}", vec!["."], Some(true));
                    logs.push(String::from("."));
                } else {
                    output::print("[STDOUT] {}", vec![message], Some(true));
                    logs.push(String::from(message))
                }
            }
            LogOutput::StdErr { message } => {
                let message = str::from_utf8(&*message).unwrap();
                output::eprint("[STDERR] {}", vec![message], Some(true));
                logs.push(String::from(message))
            }
            LogOutput::Console { message } => {
                let message = str::from_utf8(&*message).unwrap();
                output::print("[CONSOLE] {}", vec![message], Some(true));
                logs.push(String::from(message))
            }
            _ => {}
        }
    }

    let all_logs = logs.join("");

    let mut message = HashMap::<String, String>::new();

    message.insert("container_name".to_string(), container_name.to_string());
    message.insert(
        "messages".to_string(),
        format!(
            "Logs tailed for container having name \"{}\":\n\n{}",
            container_name, all_logs,
        ),
    );
    message.insert("raw_log".to_string(), all_logs);

    Ok(message)
}

async fn get_status<'a>(
    container_api_client: &DockerContainerAPIClient<Docker>,
    container_summary: &ContainerSummaryInner,
) -> Result<HashMap<String, String>, Report> {
    let container_inspect_response = container_api_client
        .client()
        .inspect_container(
            container_summary.id.as_ref().unwrap(),
            None::<InspectContainerOptions>,
        )
        .await
        .unwrap();

    let container_image = container_summary.image.as_ref().unwrap().as_str();
    let container_name = container_summary.id.as_ref().unwrap();

    if let Some(state) = container_inspect_response.state {
        if let Some(status) = state.status {
            let mut message = HashMap::<String, String>::new();
            message.insert("container_name".to_string(), container_name.to_string());
            message.insert("docker_image".to_string(), String::from(container_image));
            message.insert("raw_status".to_string(), status.to_string());
            message.insert(
                "message".to_string(),
                format!(
                    "Status provided by inspection of container having name \"{}\" and being based on \"{}\" Docker image is \"{}\"",
                    container_name.to_string(),
                    container_image,
                    status.to_string(),
                ),
            );

            return Ok(message);
        }
    }

    unreachable!()
}

pub async fn inspect_container_status<'a>(
    container_api_client: &DockerContainerAPIClient<Docker>,
    container_name: &str,
) -> Result<HashMap<String, String>, Report> {
    let mut list_container_filters = HashMap::new();
    list_container_filters.insert("name", vec![container_name]);

    if !container_exists(container_api_client, container_name)
        .await
        .unwrap()
    {
        return Err(eyre!(
            "There is no container having name \"{}\"",
            container_name
        ));
    }

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
        _ => {
            // Handled before by checking if container exists
            unreachable!();
        }
    }
}
