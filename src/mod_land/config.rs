use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  pub repositories: Vec<Repository>,
  pub repositories_dir: Option<PathBuf>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      repositories: Vec::new(),
      repositories_dir: Some(PathBuf::from(".korabli-mod-land/repositories")),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
  pub name: String,
  pub url: String,
}
