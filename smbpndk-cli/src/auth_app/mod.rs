use crate::{auth_app::cli::Commands, cli::CommandResult};
use anyhow::{anyhow, Result};
use app_oten::{create_auth_app, delete_auth_app, get_auth_app, get_auth_apps};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use smbpndk_model::AuthAppCreate;
use smbpndk_utils::{get_config, write_config};
use spinners::Spinner;

pub(crate) mod cli;

pub async fn process_auth_app(commands: Commands) -> Result<CommandResult> {
    match commands {
        Commands::New {} => {
            let app_name = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("App name")
                .interact()
                .unwrap();
            let description = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Description")
                .interact()
                .unwrap();

            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Creating an auth app...").green().bold().to_string(),
            );

            match create_auth_app(AuthAppCreate {
                name: app_name.clone(),
                description: description.clone(),
            })
            .await
            {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("An auth app created: {app_name}."),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: format!("Failed to create an auth app: {app_name}."),
                    })
                }
            }
        }
        Commands::List {} => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match get_auth_apps().await {
                Ok(auth_apps) => {
                    println!("auth_apps: {auth_apps:#?}");
                    println!(
                        "{0: <5} | {1: <20} | {2: <30} | {3: <30}",
                        "ID", "Name", "Created at", "Updated at"
                    );
                    for project in auth_apps {
                        println!(
                            "{0: <5} | {1: <20} | {2: <30} | {3: <30}",
                            project.id, project.name, project.created_at, project.updated_at
                        );
                    }
                    Ok(CommandResult {
                        spinner,
                        symbol: "✅".to_owned(),
                        msg: "Showing all auth apps.".to_owned(),
                    })
                }
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to get all auth apps.".to_owned(),
                    })
                }
            }
        }
        Commands::Show { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match get_auth_app(&id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Showing auth app.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to get an auth app.".to_owned(),
                    })
                }
            }
        }
        Commands::Delete { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match delete_auth_app(id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Delete auth app succeed.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to delete auth app.".to_owned(),
                    })
                }
            }
        }
        Commands::Use { id } => {
            let mut spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Checking config file...").green().bold().to_string(),
            );

            let auth_app = get_auth_app(&id).await?;
            let mut config = get_config().await?;

            if let Some(auth_app) = config.current_auth_app {
                if auth_app.id != id {
                    spinner.stop_with_message("Found a config.".to_string());
                    let yes = Confirm::new()
                        .with_prompt(format!(
                            "Will change active auth_app to {}. Do you want to continue?",
                            &id
                        ))
                        .interact()?;
                    if !yes {
                        let spinner = Spinner::new(
                            spinners::Spinners::SimpleDotsScrolling,
                            style("Cancelling operation...").green().bold().to_string(),
                        );
                        return Ok(CommandResult {
                            spinner,
                            symbol: "✅".to_owned(),
                            msg: "Operation cancelled.".to_string(),
                        });
                    }
                }
            }

            config.current_auth_app = Some(auth_app);
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Saving config...").green().bold().to_string(),
            );
            match write_config(config) {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("Using auth_app: {:?}", &id),
                }),
                Err(_) => {
                    let error = anyhow!("Failed while writing config.");
                    Err(error)
                }
            }
        }
    }
}
