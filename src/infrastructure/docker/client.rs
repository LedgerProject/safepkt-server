use crate::application::project::scaffold::{get_scaffolded_project_directory, prefix_hash};
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

fn get_verification_command<'a>(
    source_hash: &'a str,
    bitcode: &'a str,
    mut command: Vec<&'a str>,
) -> Vec<&'a str> {
    command.push("cargo");
    command.push("verify");
    command.push("-v");
    command.push("--bin");
    command.push(source_hash);
    command.push("-o");
    command.push(bitcode);

    command
}

fn get_configuration<'a>(
    source_hash: &'a str,
    docker_image: &'a str,
    prefixed_hash: &'a str,
    bitcode: &'a str,
) -> Result<Config<&'a str>, Report> {
    let rvt_directory = get_rvt_directory()?;

    let host_config = HostConfig {
        auto_remove: Some(false),
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

    let command = Vec::<&'a str>::new();
    let command = get_verification_command(prefixed_hash, bitcode, command);

    Ok(Config {
        cmd: Some(command),
        host_config: Some(host_config),
        image: Some(docker_image),
        user: Some("1000:1000"),
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

    let prefixed_hash = prefix_hash(source_hash);
    let bitcode = format!("{}.bc", source_hash);
    let configuration = get_configuration(
        source_hash.into(),
        &docker_image.as_str(),
        prefixed_hash.as_str(),
        bitcode.as_str(),
    )?;

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

    start_static_analysis_container(&docker, source_hash).await?;

    Ok(())
}
