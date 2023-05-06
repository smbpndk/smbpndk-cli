use crate::{account, project};
use clap::{Parser, Subcommand};

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
    #[clap(about = "Manage your account.")]
    Account {
        #[clap(subcommand)]
        command: account::cli::Commands,
    },

    #[clap(about = "Manage your projects. Add, show, delete, edit. Need authentication.")]
    Project {
        #[clap(subcommand)]
        command: project::cli::Commands,
    },

    #[clap(
        about = "Manage your Oten authentication app. Add, show, delete, edit. Need authentication."
    )]
    Oten {
        #[clap(subcommand)]
        command: app_oten::cli::Commands,
    },

    #[clap(
        about = "Manage your Fun serverless app. Add, show, delete, edit. Need authentication."
    )]
    Fun {
        #[clap(subcommand)]
        command: app_fun::cli::Commands,
    },

    #[clap(
        about = "Manage your Pkt package manager app. Add, show, delete, edit. Need authentication."
    )]
    Pkt {
        #[clap(subcommand)]
        command: app_pkt::cli::Commands,
    },

    #[clap(about = "Manage your Rdb database app. Add, show, delete, edit. Need authentication.")]
    Rdb {
        #[clap(subcommand)]
        command: app_rdb::cli::Commands,
    },
}
