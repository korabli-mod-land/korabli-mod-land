use std::sync::Arc;

#[derive(Debug, Clone, thiserror::Error)]
pub enum InitRepo {
  #[error("git2::RespositoryClone: {source}")]
  Clone { source: Arc<git2::Error> },
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UpdateRepo {
  #[error("git2::RepositoryOpen: {source}")]
  Open { source: Arc<git2::Error> },
  #[error("git2::RepositoryFindRemote: {source}")]
  FindRemote { source: Arc<git2::Error> },
  #[error("git2::RepositoryFetch: {source}")]
  Fetch { source: Arc<git2::Error> },
  #[error("git2::RepositoryMerge: {source}")]
  Merge { source: Arc<git2::Error> },
}
