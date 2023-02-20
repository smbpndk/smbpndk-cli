use std::{fs::OpenOptions, path::PathBuf, str::FromStr, time::Duration};

use anyhow::{anyhow, Result};
use clap::Parser;
use console::style;
use smbpndk_cli::{
    cli::{Cli, Commands},
    constants::ERROR_EMOJI,
    login::{process_login, LoginArgs},
};
use spinners::Spinner;
use tokio::time::sleep;
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
    match run().await {
        Ok(_) => {}
        Err(e) => {
            println!("\n{} {}", ERROR_EMOJI, style(e).red());
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    let log_level_error: Result<()> = Err(anyhow!(
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
        Commands::Login { username, password } => {
            let mut spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Logging in...").green().bold().to_string(),
            );
            let join_handle = tokio::spawn(async move {
                let _future = process_login(LoginArgs { username, password }).await;
                sleep(Duration::from_millis(5000)).await;
            });

            join_handle.await?;
            spinner.stop_and_persist("✅", style("You are logged in!").green().bold().to_string())
        }
        Commands::Signup { username, password } => {
            println!("Signup: {}, {}", username, password);
        }
    }

    Ok(())
}
