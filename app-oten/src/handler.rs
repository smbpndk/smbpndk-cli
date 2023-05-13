use crate::{cli::Commands, create_oten_app, delete_oten_app, get_oten_app, get_oten_apps};
use anyhow::{anyhow, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use smbpndk_model::{
    create_params::{self, AppCreate},
    CommandResult,
};
use smbpndk_utils::{get_config, write_config};
use spinners::Spinner;

pub async fn process_oten_app(commands: Commands) -> Result<CommandResult> {
    match commands {
        Commands::New {} => {
            let config = get_config().await?;
            if config.current_project.is_none() {
                return Err(anyhow!(
                    "No project selected. Please select a project first. Run `smb project use <id>`."
                ));
            }

            let app_name = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("App name")
                .interact()
                .unwrap();

            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Creating an Oten app...").green().bold().to_string(),
            );

            match create_oten_app(create_params::Oten {
                oten_app: AppCreate {
                    name: app_name.clone(),
                    project_id: config.current_project.unwrap().id,
                },
            })
            .await
            {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("An oten app created: {app_name}."),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: format!("Failed to create an oten app: {app_name}."),
                    })
                }
            }
        }
        Commands::List {} => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match get_oten_apps().await {
                Ok(oten_apps) => {
                    println!("oten_apps: {oten_apps:#?}");
                    println!(
                        "{0: <10} | {1: <30} | {2: <10} | {3: <10}",
                        "ID", "Name", "Created at", "Updated at"
                    );
                    for project in oten_apps {
                        let id = project.id.split('-').collect::<Vec<&str>>()[0].to_owned();
                        let created_at = project.created_at.date_naive();
                        let updated_at = project.updated_at.date_naive();
                        println!(
                            "{0: <10} | {1: <30} | {2: <10} | {3: <10}",
                            id, project.name, created_at, updated_at
                        );
                    }
                    Ok(CommandResult {
                        spinner,
                        symbol: "✅".to_owned(),
                        msg: "Showing all oten apps.".to_owned(),
                    })
                }
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to get all oten apps.".to_owned(),
                    })
                }
            }
        }
        Commands::Show { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match get_oten_app(&id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Showing oten app.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to get an oten app.".to_owned(),
                    })
                }
            }
        }
        Commands::Delete { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match delete_oten_app(id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Delete oten app succeed.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to delete oten app.".to_owned(),
                    })
                }
            }
        }
        Commands::Use { id } => {
            let mut spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Checking config file...").green().bold().to_string(),
            );

            let oten_app = get_oten_app(&id).await?;
            let mut config = get_config().await?;

            if let Some(oten_app) = config.current_oten_app {
                if oten_app.id != id {
                    spinner.stop_with_message("Found a config.".to_string());
                    let yes = Confirm::new()
                        .with_prompt(format!(
                            "Will change active oten_app to {}. Do you want to continue?",
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

            config.current_oten_app = Some(oten_app);
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Saving config...").green().bold().to_string(),
            );
            match write_config(config) {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("Using oten_app: {:?}", &id),
                }),
                Err(_) => {
                    let error = anyhow!("Failed while writing config.");
                    Err(error)
                }
            }
        }
    }
}
