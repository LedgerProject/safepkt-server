use crate::domain;
use crate::infra;
use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use color_eyre::Report;
use domain::program_verification::*;
use infra::display;
use infra::file_system::save_content_in_file_system;
use infra::PROGRAM_FUZZING;
use infra::PROGRAM_VERIFICATION;
use std::fs;
use std::path::Path;
use std::{thread, time};

pub const ARGUMENT_SOURCE: &str = "source";
pub const OPTION_WITH_FUZZING: &str = "fuzz";

pub const SUBCOMMAND_NAME_VERIFY_PROGRAM: &str = "verify_program";

pub fn verify_program_subcommand(version: &str) -> App {
    return App::new(SUBCOMMAND_NAME_VERIFY_PROGRAM)
        .about("Verify program")
        .version(version)
        .arg(
            Arg::new(OPTION_WITH_FUZZING)
                .short('f')
                .long(OPTION_WITH_FUZZING)
                .about("Fuzz test program by relying on \"project-oak/rvt\" propverify crate")
                .takes_value(false),
        )
        .arg(
            Arg::new(ARGUMENT_SOURCE)
                .short('s')
                .long(ARGUMENT_SOURCE)
                .about("Path to rust-based smart contract (e.g. https://github.com/paritytech/ink/blob/v2.1.0/examples/erc721/src/lib.rs)")
                .takes_value(true),
        );
}

async fn verify_program(source_path: &str, optional_fuzzing: Option<bool>) -> Result<(), Report> {
    let content = fs::read_to_string(source_path)?;

    let (_, project_id) = save_content_in_file_system(base64::encode(content).as_bytes())
        .expect("Can not save rust-based source in the file system.");

    let with_fuzzing = optional_fuzzing.unwrap();
    let mut step: String = String::from(PROGRAM_VERIFICATION).clone();
    if with_fuzzing {
        step = String::from(PROGRAM_FUZZING).clone()
    }

    let target = VerificationTarget::new(step.as_str(), project_id.as_str());
    let verification = SmartContractVerification::new(target);

    verification.run_step().await?;

    display::output::print("{}", vec![""], None);

    loop {
        let progress = verification.step_progress().await?;

        if progress.get("raw_status").unwrap() != "running" {
            display::output::print("{}", vec![""], None);

            break;
        } else {
            display::output::print("{}", vec!["."], Some(true));
        }

        let duration = time::Duration::from_millis(2000);
        thread::sleep(duration);
    }

    verification.step_report().await?;

    Ok(())
}

pub async fn run_verify_program_subcommand(source_path_matches: &ArgMatches) -> Result<(), Report> {
    if !source_path_matches.is_present(ARGUMENT_SOURCE) {
        display::output::eprint(
            "A --{} argument (absolute path to smart contract) is required.",
            vec![ARGUMENT_SOURCE],
            None,
        );
    }

    let with_fuzzing = source_path_matches.is_present(OPTION_WITH_FUZZING);

    if let Some(source_path) = source_path_matches.value_of(ARGUMENT_SOURCE) {
        let source = Path::new(source_path);
        if !source.exists() || source.is_dir() {
            display::output::eprint("Invalid path to rust-based smart contract.", vec![], None);
        } else {
            verify_program(source_path, Some(with_fuzzing)).await?;
        }
    }

    Ok(())
}
