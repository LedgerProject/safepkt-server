use anyhow::Result;
use clap::{App, AppSettings, ArgMatches};
use color_eyre::Report;
use safepkt_backend::app::command;
use safepkt_backend::app::middleware;
use safepkt_backend::infra::display;
use safepkt_backend::infra::sigpipe;

pub const VERSION: &str = "0.2.1";

fn configure() -> ArgMatches<'static> {
    let app = App::new("safepkt")
        .setting(AppSettings::ColorAuto)
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

    let matches = configure();

    if let Some(_) = matches.subcommand_matches(command::SUBCOMMAND_NAME_VERIFY_PROGRAM) {
        command::run_verify_program_subcommand().await?;

        return Ok(());
    }

    display::output::eprint(
        "Pass --help flag to this command to print help information",
        vec![],
    );

    Ok(())
}
