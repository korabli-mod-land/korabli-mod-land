use super::repository_updater;

#[derive(Debug, thiserror::Error)]
pub enum SearchMod {
  #[error("EnsureRepositoriesDir: {}", source)]
  EnsureRepositoriesDir { source: EnsureRepositoriesDir },
  #[error("ReadDir Io: {source}")]
  ReadDir { source: std::io::Error },
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum FetchRepository {
  #[error("EnsureRepositoriesDir: {}", source)]
  EnsureRepositoriesDir { source: EnsureRepositoriesDir },
  #[error("RepositoryUpdater::UpdateRepo: {source}")]
  RepositoryUpdaterUpdateRepo {
    source: repository_updater::error::UpdateRepo,
  },
  #[error("RepositoryDirIsNotDir")]
  RepositoryDirIsNotDir,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum EnsureRepositoriesDir {}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TryFromConfig {}

#[derive(Debug, thiserror::Error)]
pub enum GetModInfoById {
  #[error("NotFound: {id}")]
  NotFound { id: String },
  #[error("fs::TryExists: {source}")]
  FsTryExists { source: std::io::Error },
  #[error("fs::Read: {source}")]
  FsRead { source: std::io::Error },
  #[error("toml::Deserialize: {source}")]
  TomlDeserialize { source: toml::de::Error },
}
