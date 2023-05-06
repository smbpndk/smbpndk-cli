use crate::{cli::Commands, create_pkt_app, delete_pkt_app, get_pkt_app, get_pkt_apps};
use anyhow::{anyhow, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use smbpndk_model::{AppCreate, CommandResult};
use smbpndk_utils::{get_config, write_config};
use spinners::Spinner;

pub async fn process_pkt_app(commands: Commands) -> Result<CommandResult> {
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
                style("Creating a Pkt app...").green().bold().to_string(),
            );

            match create_pkt_app(AppCreate {
                name: app_name.clone(),
                description: description.clone(),
            })
            .await
            {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("Pkt app created: {app_name}."),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: format!("Failed to create a Pkt app: {app_name}."),
                    })
                }
            }
        }
        Commands::List {} => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match get_pkt_apps().await {
                Ok(apps) => {
                    println!("apps: {apps:#?}");
                    println!(
                        "{0: <10} | {1: <30} | {2: <10} | {3: <10}",
                        "ID", "Name", "Created at", "Updated at"
                    );
                    for project in apps {
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
                        msg: "Showing all Pkt apps.".to_owned(),
                    })
                }
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to get all Pkt apps.".to_owned(),
                    })
                }
            }
        }
        Commands::Show { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match get_pkt_app(&id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Showing Pkt app.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to get an Pkt app.".to_owned(),
                    })
                }
            }
        }
        Commands::Delete { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            match delete_pkt_app(id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: "Delete Pkt app succeed.".to_owned(),
                }),
                Err(e) => {
                    println!("Error: {e:#?}");
                    Ok(CommandResult {
                        spinner,
                        symbol: "😩".to_owned(),
                        msg: "Failed to delete Pkt app.".to_owned(),
                    })
                }
            }
        }
        Commands::Use { id } => {
            let mut spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Checking config file...").green().bold().to_string(),
            );

            let pkt_app = get_pkt_app(&id).await?;
            let mut config = get_config().await?;

            if let Some(pkt) = config.current_pkt_app {
                if pkt.id != id {
                    spinner.stop_with_message("Found a config.".to_string());
                    let yes = Confirm::new()
                        .with_prompt(format!(
                            "Will change active Pkt app to {}. Do you want to continue?",
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

            config.current_pkt_app = Some(pkt_app);
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Saving config...").green().bold().to_string(),
            );
            match write_config(config) {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("Using Pkt app: {:?}", &id),
                }),
                Err(_) => {
                    let error = anyhow!("Failed while writing config.");
                    Err(error)
                }
            }
        }
    }
}
