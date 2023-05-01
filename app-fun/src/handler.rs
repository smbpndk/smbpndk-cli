use crate::cli::Commands;
use anyhow::Result;
use console::style;
use smbpndk_model::CommandResult;
use spinners::Spinner;

pub async fn process_fun_app(commands: Commands) -> Result<CommandResult> {
    match commands {
        Commands::New {} => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Creating a Fun app...").green().bold().to_string(),
            );
            Ok(CommandResult {
                spinner,
                symbol: "".to_owned(),
                msg: "".to_owned(),
            })
        }
        Commands::List {} => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Listing Fun apps...").green().bold().to_string(),
            );
            Ok(CommandResult {
                spinner,
                symbol: "".to_owned(),
                msg: "".to_owned(),
            })
        }
        Commands::Show { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style(format!("Showing Fun app {id}..."))
                    .green()
                    .bold()
                    .to_string(),
            );
            Ok(CommandResult {
                spinner,
                symbol: "".to_owned(),
                msg: "".to_owned(),
            })
        }
        Commands::Delete { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style(format!("Deleting Fun app {id}..."))
                    .green()
                    .bold()
                    .to_string(),
            );
            Ok(CommandResult {
                spinner,
                symbol: "".to_owned(),
                msg: "".to_owned(),
            })
        }
        Commands::Use { id } => {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style(format!("Using Fun app {id}..."))
                    .green()
                    .bold()
                    .to_string(),
            );
            Ok(CommandResult {
                spinner,
                symbol: "".to_owned(),
                msg: "".to_owned(),
            })
        }
    }
}
