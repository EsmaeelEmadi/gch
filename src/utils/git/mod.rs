mod diff;
pub use diff::diff;

mod is_git_repo;
pub use is_git_repo::is_git_repo;

mod get_editor;
pub use get_editor::get_editor;

mod clean_commit_message;
pub use clean_commit_message::clean_commit_message;

mod create_git_commit;
pub use create_git_commit::create_git_commit;

pub mod hooks;
