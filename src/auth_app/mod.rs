use crate::{auth_app::cli::Commands, util::CommandResult};
use anyhow::Result;

pub(crate) mod cli;

pub async fn process_auth_app(commands: Commands) -> Result<CommandResult> {
    match commands {
        Commands::New {} => {
            todo!()
        }
        Commands::List {} => {
            todo!()
        }
        Commands::Show { id } => {
            todo!()
        }
        Commands::Delete { id } => {
            todo!()
        }
        Commands::Use { id } => {
            todo!()
        }
    }
}
