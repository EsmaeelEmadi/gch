use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None )]
pub struct CommitArgs {
    #[arg(short = 'd', long)]
    pub description: Option<String>,
}
