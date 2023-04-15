use std::{fs::OpenOptions, io::Write};

use crate::{auth_app::cli::Commands, debug, util::CommandResult};
use anyhow::{anyhow, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use smbpndk_model::{AuthAppCreate, Config};
use smbpndk_networking::{create_auth_app, delete_auth_app, get_auth_app, get_auth_apps};
use spinners::Spinner;

pub(crate) mod cli;

pub async fn process_auth_app(commands: Commands) -> Result<CommandResult> {
    match commands {
        Commands::New {} => {
            let project_name = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .interact()
                .unwrap();
            let description = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Description")
                .interact()
                .unwrap();

            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Creating project...").green().bold().to_string(),
            );

            match create_auth_app(AuthAppCreate {
                name: project_name.clone(),
                description: description.clone(),
            })
            .await
            {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("Creating a project {project_name}."),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "❌".to_owned(),
                        msg: format!("Failed to create a project {project_name}."),
                    })
                }
            }
        }
        Commands::List {} => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );

            // Get all
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
                        msg: "Showing all projects.".to_owned(),
                    })
                }
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "❌".to_owned(),
                        msg: "Failed to get all projects.".to_owned(),
                    })
                }
            }
        }
        Commands::Show { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            // Get Detail
            match get_auth_app(id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Showing all projects.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "❌".to_owned(),
                        msg: "Failed to get all projects.".to_owned(),
                    })
                }
            }
        }
        Commands::Delete { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            // Get Detail
            match delete_auth_app(id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Showing all projects.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "❌".to_owned(),
                        msg: "Failed to get all projects.".to_owned(),
                    })
                }
            }
        }
        Commands::Use { id } => {
            let project = get_auth_app(id).await?;

            let config = Config {
                current_project: None,
                current_auth_app: Some(project),
            };

            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match home::home_dir() {
                Some(path) => {
                    debug!(path.to_str().unwrap());
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open([path.to_str().unwrap(), "/.smb/config.json"].join(""))?;
                    let json = serde_json::to_string(&config)?;
                    file.write_all(json.as_bytes())?;

                    Ok(CommandResult {
                        spinner,
                        symbol: "✅".to_owned(),
                        msg: "Use project successful.".to_string(),
                    })
                }
                None => {
                    let error = anyhow!("Failed to get home directory.");
                    Err(error)
                }
            }
        }
    }
}
