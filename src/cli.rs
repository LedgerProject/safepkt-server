use anyhow::Result;
use clap::{App, ArgMatches};
use color_eyre::Report;
use safepkt_backend::app::command;
use safepkt_backend::app::middleware;
use safepkt_backend::infra::display;
use safepkt_backend::infra::sigpipe;
use std::env;

pub const VERSION: &str = "0.2.1";

fn configure() -> ArgMatches {
    let app = App::new("safepkt")
        .version(VERSION)
        .author("CJDNS SASU")
        .about("Rust-based smart contract verification")
        .subcommand(command::verify_program_subcommand(VERSION));

    app.get_matches()
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    // Prevent "Broken pipe" messages when passing standard display to other commands
    let _ = sigpipe::reset_signal_pipe_handler();

    middleware::logger::setup()?;
    dotenv::dotenv().ok();

    env::set_var("CLI", "true");

    let matches = configure();

    if let Some(source_path_matches) =
        matches.subcommand_matches(command::SUBCOMMAND_NAME_VERIFY_PROGRAM)
    {
        command::run_verify_program_subcommand(source_path_matches).await?;

        return Ok(());
    }

    display::output::eprint(
        "Pass --help flag to this command to print help information",
        vec![],
        None,
    );

    Ok(())
}
