use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitSection {
    pub description: Option<String>,
    pub types: Option<HashMap<String, String>>,
    pub scopes: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
}

impl CommitSection {
    /// Generate a markdown string from the commit section.
    pub fn commit_guideline_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Commit Configuration\n\n");
        md.push_str(&format!("**Description:** {:?}\n\n", self.description));

        match &self.scopes {
            None => (),
            Some(scopes) => {
                md.push_str("## Scopes\n\n");
                for scope in scopes {
                    md.push_str(&format!("- {:?}\n", scope));
                }
                md.push('\n');
            }
        }

        match &self.types {
            None => (),
            Some(types) => {
                md.push_str("## Types\n\n");
                for (commit_type, desc) in types {
                    md.push_str(&format!("- **{}**: {}\n", commit_type, desc));
                }
                md.push('\n');
            }
        }

        match &self.rules {
            None => (),
            Some(rules) => {
                md.push_str("## Rules\n\n");
                for rule in rules {
                    md.push_str(&format!("- {:?}\n", rule));
                }
                md.push('\n');
            }
        }

        match &self.examples {
            None => (),
            Some(examples) => {
                md.push_str("## Examples\n\n");
                for example in examples {
                    md.push_str(&format!("- {:?}\n", example));
                }
                md.push('\n');
            }
        }

        md
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeRequestSection {
    pub descritption: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub pre_commit: Option<Vec<String>>,
    pub commit: Option<CommitSection>,
    pub merge_request: Option<MergeRequestSection>,
}
