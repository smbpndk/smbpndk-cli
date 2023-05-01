use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Add new Fun app.")]
    New {},

    #[clap(about = "List all your Fun apps.")]
    List {},

    #[clap(about = "Show detail of a Fun app.")]
    Show {
        /// Project Id
        #[clap(short, long, required = true)]
        id: String,
    },

    #[clap(about = "Delete a Fun app.")]
    Delete {
        /// Project name
        #[clap(short, long, required = true)]
        id: String,
    },

    #[clap(about = "Use Fun app for current CLI session.")]
    Use {
        #[clap(short, long, required = true)]
        id: String,
    },
}
