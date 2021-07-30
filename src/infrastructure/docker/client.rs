use crate::application::project::scaffold::{get_scaffolded_project_directory, prefix_hash};
use anyhow::Result;
use bollard::container::{
    Config, CreateContainerOptions, ListContainersOptions, RemoveContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::*;
use bollard::Docker;
use color_eyre::Report;
use futures::StreamExt;
use std::collections::HashMap;
use std::default::Default;
use std::env;
use tracing::info;

static TARGET_RVT_DIRECTORY: &str = "/home/rust-verification-tools";
static TARGET_SOURCE_DIRECTORY: &str = "/source";

fn get_rvt_directory() -> Result<String, Report> {
    let source_directory = env::var("RVT_DIRECTORY")?;
    Ok(source_directory)
}

fn get_rvt_docker_image() -> Result<String, Report> {
    let docker_image = env::var("RVT_DOCKER_IMAGE")?;
    Ok(docker_image)
}

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

async fn remove_existing_container(docker: &Docker, source_hash: &str) -> Result<(), Report> {
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

fn get_configuration<'a>(
    source_hash: &'a str,
    docker_image: &'a str,
) -> Result<Config<&'a str>, Report> {
    let rvt_directory = get_rvt_directory()?;

    let host_config = HostConfig {
        mounts: Some(vec![
            Mount {
                target: Some(TARGET_SOURCE_DIRECTORY.to_string()),
                source: Some(get_scaffolded_project_directory(source_hash)),
                typ: Some(MountTypeEnum::BIND),
                consistency: Some(String::from("default")),
                ..Default::default()
            },
            Mount {
                target: Some(TARGET_RVT_DIRECTORY.to_string()),
                source: Some(rvt_directory),
                typ: Some(MountTypeEnum::BIND),
                consistency: Some(String::from("default")),
                ..Default::default()
            },
        ]),
        network_mode: Some(String::from("host")),
        ..Default::default()
    };

    Ok(Config {
        entrypoint: Some(vec!["tail", "-f", "/dev/null"]),
        host_config: Some(host_config),
        image: Some(docker_image),
        working_dir: Some(TARGET_SOURCE_DIRECTORY),
        ..Default::default()
    })
}

async fn start_static_analysis_container<'a>(
    docker: &'a Docker,
    source_hash: &'a str,
) -> Result<(&'a Docker, String), Report> {
    remove_existing_container(docker, source_hash).await?;

    let docker_image = get_rvt_docker_image()?;
    let configuration = get_configuration(source_hash.into(), &docker_image.as_str())?;

    let id = docker
        .create_container(
            Some(CreateContainerOptions { name: source_hash }),
            configuration,
        )
        .await?
        .id;

    docker.start_container::<String>(&id, None).await?;

    Ok((docker, id))
}

pub async fn run_static_analysis(source_hash: &str) -> Result<(), Report> {
    let docker = Docker::connect_with_socket_defaults()?;

    let (docker, id) = start_static_analysis_container(&docker, source_hash).await?;

    let prefixed_source_hash = prefix_hash(source_hash);

    let exec = docker
        .create_exec(
            &id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec![
                    "cargo",
                    "verify",
                    "-v",
                    "--bin",
                    prefixed_source_hash.as_str(),
                    "-o",
                    format!("{}.bc", prefixed_source_hash).as_str(),
                ]),
                ..Default::default()
            },
        )
        .await?
        .id;

    if let StartExecResults::Attached { mut output, .. } = docker.start_exec(&exec, None).await? {
        while let Some(Ok(msg)) = output.next().await {
            info!("{}", msg);
        }

        return Ok(());
    }

    unreachable!();
}
