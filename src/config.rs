use std::{
    fs::{self},
    path::PathBuf,
};

use expanduser::expanduser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: expanduser("~/ttrack").unwrap(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let Ok(content) = fs::read_to_string("~/ttrack.json") else {
            return Default::default();
        };
        match serde_json::from_str(content.as_str()) {
            Ok(config) => return config,
            Err(error) => {
                eprintln!("ERROR could not parse config ttrack.json");
                panic!("{}", error);
            }
        };
    }

    pub fn database_path(&self) -> PathBuf {
        self.path.join("database.db")
    }
}
