use crate::mod_land;

#[derive(Debug, Clone, thiserror::Error)]
pub enum EnsureDirs {}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReadModLandConfig {}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Serve {
  #[error("ReadModLandConfig: {source}")]
  ReadModLandConfig { source: ReadModLandConfig },
  #[error("EnsureDirs: {source}")]
  EnsureDirs { source: EnsureDirs },
  #[error("ModLand::TryFromConfig: {source}")]
  ModLandTryFromConfig {
    source: mod_land::error::TryFromConfig,
  },
}
