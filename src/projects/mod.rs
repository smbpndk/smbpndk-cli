mod crud;

use std::{fs::OpenOptions, io::Write};

use anyhow::{anyhow, Result};
use clap::Subcommand;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use spinners::Spinner;

use crate::{debug, util::CommandResult};

use self::crud::{create_project, delete_project, get_all, get_project, Config, ProjectCreate};

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Add new project.")]
    New {},

    #[clap(about = "List all your projects.")]
    List {},

    #[clap(about = "Show detail of a project.")]
    Show {
        /// Project Id
        #[clap(short, long, required = true)]
        id: String,
    },

    #[clap(about = "Delete a project.")]
    Delete {
        /// Project name
        #[clap(short, long, required = true)]
        id: String,
    },

    #[clap(about = "Use project for current CLI session.")]
    Use {
        #[clap(short, long, required = true)]
        id: String,
    },
}

pub async fn process_projects(commands: Commands) -> Result<CommandResult> {
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

            match create_project(ProjectCreate {
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
            match get_all().await {
                Ok(projects) => {
                    println!("Projects: {projects:#?}");
                    println!(
                        "{0: <5} | {1: <20} | {2: <30} | {3: <30}",
                        "ID", "Name", "Created at", "Updated at"
                    );
                    for project in projects {
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
            match get_project(id).await {
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
                style("Deleting project...").green().bold().to_string(),
            );
            match delete_project(id).await {
                Ok(_) => Ok(CommandResult {
                    spinner,
                    symbol: "✅".to_owned(),
                    msg: format!("Project deleted."),
                }),
                Err(e) => {
                    let error = anyhow!("Failed to delete project. {e}");
                    return Err(error);
                }
            }
        }
        Commands::Use { id } => {
            let project = get_project(id).await?;

            let config = Config {
                current_project: Some(project),
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
                        msg: format!("Use project successful."),
                    })
                }
                None => {
                    let error = anyhow!("Failed to get home directory.");
                    return Err(error);
                }
            }
        }
    }
}
