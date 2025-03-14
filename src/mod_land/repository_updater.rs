use std::path::Path;

use async_trait::async_trait;
use url::Url;

pub(in crate::mod_land) mod error;
pub(in crate::mod_land) mod impls;

#[async_trait]
pub trait RepositoryUpdater {
  fn can_update(&self, url: &Url) -> bool;
  async fn init_repo(&self, url: Url, path: &Path) -> Result<(), error::InitRepo>;
  async fn update_repo(&self, url: Url, path: &Path) -> Result<(), error::UpdateRepo>;
}
