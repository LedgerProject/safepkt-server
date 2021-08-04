use crate::domain::value_object::*;
use crate::infra::project_scaffold::{format_directory_path_to_scaffold, format_project_name};
use crate::infra::verification_runtime::docker::DockerContainerAPIClient;
use anyhow::Result;
use bollard::container::{Config, CreateContainerOptions};
use bollard::{models::*, Docker};
use color_eyre::Report;
use std::env;
use tracing::info;

pub static TARGET_RVT_DIRECTORY: &str = "/home/rust-verification-tools";
static TARGET_SOURCE_DIRECTORY: &str = "/source";

fn get_rvt_directory() -> Result<String, Report> {
    let source_directory = env::var("RVT_DIRECTORY")?;
    Ok(source_directory)
}

fn get_rvt_container_image() -> Result<String, Report> {
    let container_image = env::var("RVT_DOCKER_IMAGE")?;
    Ok(container_image)
}

pub fn llvm_bitcode_generation_cmd_provider() -> StepProvider {
    |prefixed_hash: &str, bitcode: &str| -> String {
        format!("cargo verify -v --bin {} -o {}", prefixed_hash, bitcode)
    }
}

pub fn symbolic_execution_cmd_provider() -> StepProvider {
    |_: &str, bitcode: &str| -> String {
        format!("klee --libc=klee --posix-runtime --disable-verify {} --sym-args 0 3 10 --sym-files 2 8", bitcode)
    }
}

fn get_configuration<'a>(
    command_parts: Vec<&'a str>,
    container_image: &'a str,
    project_id: &'a str,
) -> Result<Config<&'a str>, Report> {
    let rvt_directory = get_rvt_directory()?;

    let host_config = HostConfig {
        auto_remove: Some(false),
        mounts: Some(vec![
            Mount {
                target: Some(TARGET_SOURCE_DIRECTORY.to_string()),
                source: Some(format_directory_path_to_scaffold(project_id)),
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
        cmd: Some(command_parts),
        host_config: Some(host_config),
        image: Some(container_image),
        user: Some("1000:1000"),
        working_dir: Some(TARGET_SOURCE_DIRECTORY),
        ..Default::default()
    })
}

fn get_bitcode_filename(project_id: &str) -> String {
    format!("{}.bc", project_id)
}

pub async fn start_container(
    container_api_client: &DockerContainerAPIClient<Docker>,
    container_name: String,
    project_step: &StepInVerificationPlan<'_>,
) -> Result<(), Report> {
    let project_id = project_step.project_id().clone();
    let step = project_step.step;

    let container_image = get_rvt_container_image()?;
    let prefixed_hash = format_project_name(project_id.as_str());
    let prefixed_hash = prefixed_hash.as_str();

    let bitcode_file_name = get_bitcode_filename(project_id.as_str());
    let bitcode_file_name = bitcode_file_name.as_str();

    let command: String = step.step_provider()(prefixed_hash, bitcode_file_name);
    let command = command.as_str();
    let command_parts = command.split(" ").collect::<Vec<&str>>();

    let configuration =
        get_configuration(command_parts, container_image.as_str(), project_id.as_str())?;

    info!(
        "About to start container with name {} based on image {}",
        container_name.as_str(),
        container_name
    );

    let id = container_api_client
        .client()
        .create_container(
            Some(CreateContainerOptions {
                name: container_name.as_str(),
            }),
            configuration,
        )
        .await?
        .id;

    container_api_client
        .client()
        .start_container::<String>(&id, None)
        .await?;

    Ok(())
}
