use std::sync::Arc;

#[derive(Debug, Clone, thiserror::Error)]
pub enum InitRepo {
  #[error("git2::RespositoryClone: {source}")]
  Git2RepositoryClone { source: Arc<git2::Error> },
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UpdateRepo {
  #[error("git2::RespositoryFastForward: {source}")]
  Git2RepositoryFastForward { source: Arc<git2::Error> },
  #[error("git2::RepositoryOpen: {source}")]
  Git2RepositoryOpen { source: Arc<git2::Error> },
  #[error("git2::RepositoryFindRemote: {source}")]
  Git2RepositoryFindRemote { source: Arc<git2::Error> },
  #[error("git2::RepositoryFetch: {source}")]
  Git2RepositoryFetch { source: Arc<git2::Error> },
  #[error("git2::RepositoryMerge: {source}")]
  Git2RepositoryMerge { source: Arc<git2::Error> },
}
