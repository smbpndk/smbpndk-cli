use anyhow::{anyhow, Result};
use clap::Parser;
use console::style;
use dotenv::dotenv;
use std::{fs::OpenOptions, path::PathBuf, str::FromStr};

use smbpndk_cli::{
    account::{
        forgot::process_forgot,
        login::{process_login, process_logout},
        signup::process_signup,
    },
    auth_app::process_auth_app,
    cli::{Cli, Commands},
    projects::process_projects,
    util::CommandResult,
};

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::LevelFilter, prelude::*, EnvFilter};

fn setup_logging(level: Option<EnvFilter>) -> Result<()> {
    // Log in the current directory
    let log_path = PathBuf::from("smbpndk-cli.log");

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(log_path)
        .unwrap();

    let env_filter = if let Some(filter) = level {
        filter
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"))
    };

    let formatting_layer = BunyanFormattingLayer::new("smb".into(), file);
    let level_filter = LevelFilter::from_str(&env_filter.to_string())?;

    let subscriber = tracing_subscriber::registry()
        .with(formatting_layer.with_filter(level_filter))
        .with(JsonStorageLayer);

    set_global_default(subscriber).expect("Failed to set global default subscriber");

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    match run().await {
        Ok(result) => {
            let mut spinner = result.spinner;
            spinner.stop_and_persist(&result.symbol, result.msg);
            std::process::exit(1);
        }
        Err(e) => {
            println!(
                "\n{} {}",
                style("✘".to_string()).for_stderr().red(),
                style(e).red()
            );
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<CommandResult> {
    let cli = Cli::parse();

    let log_level_error: Result<CommandResult> = Err(anyhow!(
        "Invalid log level: {:?}.\n Valid levels are: trace, debug, info, warn, and error.",
        cli.log_level
    ));

    if let Some(user_filter) = cli.log_level {
        let filter = match EnvFilter::from_str(&user_filter) {
            Ok(filter) => filter,
            Err(_) => return log_level_error,
        };
        setup_logging(Some(filter))?;
    } else {
        setup_logging(None)?;
    }

    match cli.command {
        Commands::Login {} => process_login().await,
        Commands::Logout {} => process_logout().await,
        Commands::Signup {} => process_signup().await,
        Commands::Forgot {} => process_forgot().await,
        Commands::Projects { command } => process_projects(command).await,
        Commands::AuthApp { command } => process_auth_app(command).await,
    }
}