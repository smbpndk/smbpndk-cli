use clap::{Parser, Subcommand};

use crate::projects;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Log level: trace, debug, info, warn, error, off
    #[clap(short, long, global = true)]
    pub log_level: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Create an account. Use your email as your username.")]
    Signup {},

    #[clap(
        about = "Login to your account. To create an account, use smb signup or visit https://smbpndk.com"
    )]
    Login {},
    #[clap(about = "Logout all session.")]
    Logout {},

    #[clap(about = "Forgot email? Use this command to reset your password.")]
    Forgot {},

    #[clap(about = "Manage your projects. Add, delete, edit. Need authentication.")]
    Projects {
        #[clap(subcommand)]
        command: projects::Commands,
    },
}
