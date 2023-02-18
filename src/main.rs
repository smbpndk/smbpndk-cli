use anyhow::Result;
use clap::Parser;
use smbpndk_cli::{
    cli::{Cli, Commands},
    login::{process_login, LoginArgs},
};

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {
            println!("Command successful.");
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Login { username, password } => {
            println!("Login: {}, {}", username, password);
            process_login(LoginArgs { username, password }).await?;
        }
    }

    Ok(())
}
