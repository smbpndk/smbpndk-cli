use anyhow::{anyhow, Result};
use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use dotenv::dotenv;
use std::{fs::OpenOptions, path::PathBuf, str::FromStr};

use smbpndk_cli::{
    account::{
        login::{process_login, LoginArgs},
        signup::{signup_with_email, signup_with_github, SignupMethod},
    },
    cli::{Cli, Commands},
    projects,
    util::CommandResult,
};
use spinners::Spinner;

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
        Commands::Login {} => {
            println!("Provide your login credentials.");
            let username = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Username")
                .interact()
                .unwrap();
            let password = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Password")
                .interact()
                .unwrap();

            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Logging in...").green().bold().to_string(),
            );

            match process_login(LoginArgs { username, password }).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "You are logged in!".to_owned(),
                }),
                Err(e) => Ok(CommandResult {
                    spinner,
                    symbol: "❌".to_owned(),
                    msg: format!("Failed to login: {e}"),
                }),
            }
        }
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
        Commands::Projects { command } => projects::process(command).await,
    }
}
