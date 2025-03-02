mod read_errors;
use read_errors::ReadConfigError;

mod read;
pub use read::read;

mod types;
pub use types::{CommitSection, Config, MergeRequestSection};
