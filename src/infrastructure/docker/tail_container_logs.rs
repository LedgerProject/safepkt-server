use anyhow::Result;
use bollard::container::{LogOutput, LogsOptions};
use bollard::Docker;
use color_eyre::Report;
use futures::stream::StreamExt;
use std::default::Default;
use std::str;
use tracing::info;

pub async fn get_container_logs(docker: &Docker, source_hash: &str) -> Result<String, Report> {
    let mut logs_stream = docker.logs(
        source_hash,
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
