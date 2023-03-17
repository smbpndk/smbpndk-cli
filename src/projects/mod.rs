mod crud;

use anyhow::Result;
use clap::Subcommand;
use console::style;
use spinners::Spinner;

use crate::util::CommandResult;

use self::crud::get_all;

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Add new project.")]
    New {
        /// Project name
        #[clap(short, long, global = true)]
        name: Option<String>,
    },

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
    let spinner = Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("Loading...").green().bold().to_string(),
    );
    match commands {
        Commands::New { name } => {
            let project_name = match name {
                Some(name) => name,
                None => "tempe-goreng".to_owned(),
            };

            Ok(CommandResult {
                spinner,
                symbol: "✅".to_owned(),
                msg: format!("Creating a project {project_name}."),
            })
        }
        Commands::List {} => {
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
        Commands::Show { name } => Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: format!("Showing project {name}."),
        }),
        Commands::Delete { name } => Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: format!("Deleting project {name}."),
        }),
    }
}
