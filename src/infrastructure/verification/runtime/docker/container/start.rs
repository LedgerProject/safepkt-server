use crate::application::project::scaffold::{get_scaffolded_project_directory, prefix_hash};
use anyhow::Result;
use bollard::container::{Config, CreateContainerOptions};
use bollard::models::*;
use bollard::Docker;
use color_eyre::Report;
use std::env;

static TARGET_RVT_DIRECTORY: &str = "/home/rust-verification-tools";
static TARGET_SOURCE_DIRECTORY: &str = "/source";

fn get_rvt_directory() -> Result<String, Report> {
    let source_directory = env::var("RVT_DIRECTORY")?;
    Ok(source_directory)
}

fn get_rvt_container_image() -> Result<String, Report> {
    let container_image = env::var("RVT_DOCKER_IMAGE")?;
    Ok(container_image)
}

fn get_verification_command<'a>(
    target_hash: &'a str,
    bitcode: &'a str,
    mut command: Vec<&'a str>,
) -> Vec<&'a str> {
    command.push("cargo");
    command.push("verify");
    command.push("-v");
    command.push("--bin");
    command.push(target_hash);
    command.push("-o");
    command.push(bitcode);

    command
}

fn get_configuration<'a>(
    container_image: &'a str,
    target_hash: &'a str,
    prefixed_hash: &'a str,
    bitcode_file_name: &'a str,
) -> Result<Config<&'a str>, Report> {
    let rvt_directory = get_rvt_directory()?;

    let host_config = HostConfig {
        auto_remove: Some(false),
        mounts: Some(vec![
            Mount {
                target: Some(TARGET_SOURCE_DIRECTORY.to_string()),
                source: Some(get_scaffolded_project_directory(target_hash)),
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
    let command = get_verification_command(prefixed_hash, bitcode_file_name, command);

    Ok(Config {
        cmd: Some(command),
        host_config: Some(host_config),
        image: Some(container_image),
        user: Some("1000:1000"),
        working_dir: Some(TARGET_SOURCE_DIRECTORY),
        ..Default::default()
    })
}

fn get_bitcode_filename(target_hash: &str) -> String {
    format!("{}.bc", target_hash)
}

pub async fn start_container(api_client: &Docker, target_hash: &str) -> Result<(), Report> {
    let container_image = get_rvt_container_image()?;

    let prefixed_hash = prefix_hash(target_hash);
    let bitcode_file_name = get_bitcode_filename(target_hash);
    let configuration = get_configuration(
        container_image.as_str(),
        target_hash.into(),
        prefixed_hash.as_str(),
        bitcode_file_name.as_str(),
    )?;

    let id = api_client
        .create_container(
            Some(CreateContainerOptions { name: target_hash }),
            configuration,
        )
        .await?
        .id;

    api_client.start_container::<String>(&id, None).await?;

    Ok(())
}
