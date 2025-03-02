use anyhow::Result;

use crate::utils::{cmd, config::Config};

pub fn pre_commits(config: &Config) -> Result<bool> {
    match &config.pre_commit {
        None => return Ok(true),
        Some(list) => {
            if list.len() == 0 {
                return Ok(true);
            } else {
                for cmd in list {
                    match cmd::run(cmd) {
                        Ok(true) => (),
                        Ok(false) => return Ok(false),
                        Err(_) => return Ok(false),
                    };
                }

                Ok(true)
            }
        }
    }
}
