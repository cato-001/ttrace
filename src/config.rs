use std::{fs, path::PathBuf};

use expanduser::expanduser;
use eyre::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: expanduser("~/.local/state/ttrack").unwrap(),
        }
    }
}

impl Config {
    pub fn load() -> eyre::Result<Self> {
        let config = fs::read_to_string("~/.ttrack.json")
            .map(|content| {
                serde_json::from_str(content.as_str())
                    .wrap_err("ERROR \"could not parse the '~/.ttrack.json' config!\"")
                    .with_context(|| content)
            })
            .unwrap_or(Ok(Self::default()))?;
        fs::create_dir_all(&config.path)?;
        Ok(config)
    }

    pub fn database_path(&self) -> PathBuf {
        self.path.join("database.db")
    }
}
