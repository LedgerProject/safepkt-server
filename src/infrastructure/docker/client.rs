use crate::application::project::scaffold::get_scaffolded_project_directory;
use anyhow::Result;
use bollard::container::{
    Config, CreateContainerOptions, ListContainersOptions, RemoveContainerOptions,
};
use bollard::models::*;
use bollard::Docker;
use color_eyre::Report;
use std::collections::HashMap;
use std::default::Default;
use std::env;

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

pub async fn container_exists(docker: &Docker, name: &str) -> Result<bool, Report> {
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

pub async fn start_static_analysis_container(source_hash: &str) -> Result<(), Report> {
    let docker = Docker::connect_with_socket_defaults()?;

    let existing_container = container_exists(&docker, source_hash).await.unwrap();

    if existing_container {
        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });

        docker.remove_container(source_hash, options).await.unwrap();
    }

    let project_directory = get_scaffolded_project_directory(source_hash);
    let rvt_directory = get_rvt_directory()?;

    let host_config = HostConfig {
        mounts: Some(vec![
            Mount {
                target: Some(String::from(TARGET_SOURCE_DIRECTORY)),
                source: Some(project_directory),
                typ: Some(MountTypeEnum::BIND),
                consistency: Some(String::from("default")),
                ..Default::default()
            },
            Mount {
                target: Some(String::from(TARGET_RVT_DIRECTORY)),
                source: Some(rvt_directory),
                typ: Some(MountTypeEnum::BIND),
                consistency: Some(String::from("default")),
                ..Default::default()
            },
        ]),
        network_mode: Some(String::from("host")),
        ..Default::default()
    };

    let docker_image = get_rvt_docker_image()?;

    let id = docker
        .create_container(
            Some(CreateContainerOptions { name: source_hash }),
            Config {
                entrypoint: Some(vec!["tail", "-f", "/dev/null"]),
                host_config: Some(host_config),
                image: Some(docker_image.as_str()),
                working_dir: Some(TARGET_SOURCE_DIRECTORY),
                ..Default::default()
            },
        )
        .await?
        .id;

    docker.start_container::<String>(&id, None).await?;

    Ok(())
}
