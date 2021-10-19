use anyhow::Result;
use clap::App;
use color_eyre::Report;

pub const SUBCOMMAND_NAME_VERIFY_PROGRAM: &str = "verify_program";

pub fn verify_program_subcommand(version: &str) -> App {
    return App::new(SUBCOMMAND_NAME_VERIFY_PROGRAM)
        .about("Verify program")
        .version(version);
}

async fn verify_program() {
    // pin_based_authorization_workflow::run_pin_based_authorization_workflow().await
}

pub async fn run_verify_program_subcommand() -> Result<(), Report> {
    verify_program().await;

    Ok(())
}
