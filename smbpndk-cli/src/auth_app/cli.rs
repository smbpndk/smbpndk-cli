use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Add new auth app.")]
    New {},

    #[clap(about = "List all your auth apps.")]
    List {},

    #[clap(about = "Show detail of a auth app.")]
    Show {
        /// Project Id
        #[clap(short, long, required = true)]
        id: String,
    },

    #[clap(about = "Delete a auth app.")]
    Delete {
        /// Project name
        #[clap(short, long, required = true)]
        id: String,
    },

    #[clap(about = "Use auth app for current CLI session.")]
    Use {
        #[clap(short, long, required = true)]
        id: String,
    },
}
