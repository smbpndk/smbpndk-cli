use anyhow::{anyhow, Result};
use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use std::{fs::OpenOptions, path::PathBuf, str::FromStr};

use smbpndk_cli::{
    account::{
        self,
        login::{process_login, process_logout, LoginArgs},
        signup::{signup_with_email, signup_with_github, SignupMethod},
    },
    cli::{Cli, Commands},
    projects,
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
        Commands::Signup {} => {
            let signup_methods = vec![SignupMethod::Email, SignupMethod::GitHub];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&signup_methods)
                .default(0)
                .interact_on_opt(&Term::stderr())
                .map(|i| signup_methods[i.unwrap()])
                .unwrap();

            match selection {
                SignupMethod::Email => signup_with_email(None).await,
                SignupMethod::GitHub => signup_with_github().await,
            }
        }
        Commands::Forgot {} => account::forgot::process().await,
        Commands::Projects { command } => projects::process(command).await,
    }
}
