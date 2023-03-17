mod crud;

use anyhow::Result;
use clap::Subcommand;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use spinners::Spinner;

use crate::util::CommandResult;

use self::crud::{create_project, get_all, ProjectCreate};

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Add new project.")]
    New {},

    #[clap(about = "List all your projects.")]
    List {},

    #[clap(about = "Show detail of a project.")]
    Show {
        /// Project name
        #[clap(short, long, required = true)]
        name: String,
    },

    #[clap(about = "Delete a project.")]
    Delete {
        /// Project name
        #[clap(short, long, required = true)]
        name: String,
    },
}

pub async fn process(commands: Commands) -> Result<CommandResult> {
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
                    println!("Error: {:#?}", e);
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
                    println!("Projects: {:#?}", projects);
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
                    println!("Error: {:#?}", e);
                    Ok(CommandResult {
                        spinner,
                        symbol: "❌".to_owned(),
                        msg: "Failed to get all projects.".to_owned(),
                    })
                }
            }
        }
        Commands::Show { name } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            Ok(CommandResult {
                spinner,
                symbol: "✅".to_owned(),
                msg: format!("Showing project {name}."),
            })
        }
        Commands::Delete { name } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Loading...").green().bold().to_string(),
            );
            Ok(CommandResult {
                spinner,
                symbol: "✅".to_owned(),
                msg: format!("Deleting project {name}."),
            })
        }
    }
}
