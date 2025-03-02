use std::process;

use crate::utils::{agents, config};
use spinners::{Spinner, Spinners};

use crate::utils::git;

use super::CommitArgs;

#[tokio::main]
pub async fn commit(args: &CommitArgs, config: &config::Config) {
    let mut spinner = Spinner::new(Spinners::Dots9, "Processing...".into());

    let diff = match git::diff() {
        Ok(diff) if !diff.is_empty() => diff,
        Ok(_) => {
            spinner.stop_with_message("No changes in Git repository".to_string());
            process::exit(1);
        }
        Err(e) => {
            spinner.stop_with_message(format!("Error getting Git diff: {}", e));
            process::exit(1);
        }
    };

    let guideline = match &config.commit {
        None => "".to_string(),
        Some(config) => config.commit_guideline_markdown(),
    };

    match git::hooks::pre_commits(&config) {
        Err(e) => {
            spinner.stop_with_message(format!("Error running pre-commits: {}", e));
            process::exit(1);
        }
        Ok(true) => {}
        Ok(false) => {
            spinner.stop_with_message("Error running pre-commits".to_string());
            process::exit(1);
        }
    }

    let response = match agents::get_commit_messsage(&args.description, &config, &guideline, &diff)
        .await
    {
        Err(e) => {
            spinner.stop_with_message(format!("Error getting commit message from agent: {:?}", e));
            process::exit(1);
        }
        Ok(o) => o,
    };

    spinner.stop_with_newline();

    if let Err(e) = git::create_git_commit(&response) {
        spinner.stop_with_message(format!("Erro creating git commit: {}", e));
        process::exit(1);
    }
    println!("Commit created successfully");
}
