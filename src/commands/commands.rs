use clap::Subcommand;

use super::CommitArgs;

#[derive(Subcommand)]
pub enum Commands {
    /// Commit your changes with a message
    Commit(CommitArgs),
}
