use std::{collections::BTreeMap, path::PathBuf};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use log::{debug, warn};
use mod_info::ModInfo;
use repository_updater::RepositoryUpdater;
use tokio::fs;
use url::Url;

pub mod config;
pub mod error;
pub mod mod_info;
mod repository_updater;

pub struct ModLand {
  repositories: Vec<(String, Url)>,
  repositories_updaters: Vec<Box<dyn RepositoryUpdater + Sync + Send>>,
  repositories_dir: PathBuf,
  fuzzy_matcher: SkimMatcherV2,
}

impl ModLand {
  pub fn try_from_config(config: config::Config) -> Result<Self, error::TryFromConfig> {
    Ok(Self {
      repositories: config
        .repositories
        .into_iter()
        .map(|repo| (repo.name, repo.url.parse().expect("invalid url")))
        .collect(),
      repositories_updaters: vec![Box::new(repository_updater::impls::git::Git::new())],
      repositories_dir: config
        .repositories_dir
        .unwrap_or(PathBuf::from(".korabli-mod-land/repositories")),
      fuzzy_matcher: SkimMatcherV2::default(),
    })
  }

  async fn ensure_repositories_dir(&mut self) -> Result<(), error::EnsureRepositoriesDir> {
    if fs::try_exists(self.repositories_dir.as_path())
      .await
      .expect("check cache dir failed")
    {
      if fs::metadata(self.repositories_dir.as_path())
        .await
        .expect("check cache dir metadata failed")
        .is_dir()
      {
        return Ok(());
      } else {
        panic!("cache dir is file");
      }
    }
    fs::create_dir_all(self.repositories_dir.as_path())
      .await
      .expect("create cache dir failed");
    Ok(())
  }

  pub async fn fetch_repository<Repo>(&mut self, repo: Repo) -> Result<bool, error::FetchRepository>
  where
    Repo: AsRef<str>,
  {
    debug!("fetching repository: {}", repo.as_ref());
    self
      .ensure_repositories_dir()
      .await
      .map_err(|err| error::FetchRepository::EnsureRepositoriesDir { source: err })?;

    let Some((name, url)) = self
      .repositories
      .iter()
      .find(|(name, _)| name == repo.as_ref())
    else {
      warn!("repository not exists: {}", repo.as_ref());
      return Ok(false);
    };

    let repo_dir = self.repositories_dir.join(name);
    debug!("repo_dir: {}", repo_dir.to_string_lossy());

    debug!("updaters");
    for updater in self.repositories_updaters.iter() {
      debug!("updater found");
      if updater.can_update(url) {
        if !fs::try_exists(repo_dir.as_path())
          .await
          .expect("check repo dir failed")
        {
          debug!("repo dir not created, try init it");
          updater
            .init_repo(url.to_owned(), repo_dir.as_path())
            .await
            .expect("init repo failed");
        } else if fs::metadata(repo_dir.as_path())
          .await
          .expect("check repo dir failed")
          .is_dir()
        {
          debug!("repo dir already created, try update it");
          updater
            .update_repo(url.to_owned(), repo_dir.as_path())
            .await
            .inspect_err(|err| {
              warn!("update repo failed: {}", err);
            })
            .map_err(|err| error::FetchRepository::RepositoryUpdaterUpdateRepo { source: err })?;
        } else {
          return Err(error::FetchRepository::RepositoryDirIsNotDir);
        }
        return Ok(true);
      }
    }

    panic!("cannot update repo because there is no updater for it");
  }

  pub async fn search_mod_by_id<ModId>(
    &mut self,
    id: ModId,
  ) -> Result<Vec<(i64, String)>, error::SearchMod>
  where
    ModId: AsRef<str>,
  {
    self
      .ensure_repositories_dir()
      .await
      .map_err(|err| error::SearchMod::EnsureRepositoriesDir { source: err })?;
    let mut matches = BTreeMap::new();
    for repo in self.repositories.iter().rev() {
      let repo_dir = self.repositories_dir.join(&repo.0);
      let mut toml_files = fs::read_dir(repo_dir)
        .await
        .map_err(|err| error::SearchMod::ReadDir { source: err })?;
      while let Some(toml_file) = toml_files
        .next_entry()
        .await
        .map_err(|err| error::SearchMod::ReadDir { source: err })?
      {
        let name = toml_file.file_name();

        if !name.to_string_lossy().ends_with(".toml") {
          continue;
        }

        let Some(score) = self
          .fuzzy_matcher
          .fuzzy_match(name.to_string_lossy().as_ref(), id.as_ref())
        else {
          continue;
        };

        matches.insert(
          score,
          name
            .to_string_lossy()
            .strip_suffix(".toml")
            .expect("huh?")
            .to_string(),
        );
      }
    }
    Ok(matches.into_iter().rev().take(5).collect())
  }

  pub async fn get_mod_info_by_id<ModId>(
    &mut self,
    id: ModId,
  ) -> Result<ModInfo, error::GetModInfoById>
  where
    ModId: AsRef<str>,
  {
    for (repo, _) in self.repositories.iter() {
      let repo_dir = self.repositories_dir.join(repo);
      let toml_file = repo_dir.join(format!("{}.toml", id.as_ref()));

      if !fs::try_exists(toml_file.as_path())
        .await
        .map_err(|err| error::GetModInfoById::FsTryExists { source: err })?
      {
        continue;
      }

      let mod_info = fs::read_to_string(toml_file.as_path())
        .await
        .map_err(|err| error::GetModInfoById::FsRead { source: err })?;
      let mod_info = toml::from_str(mod_info.as_str())
        .map_err(|err| error::GetModInfoById::TomlDeserialize { source: err })?;
      return Ok(mod_info);
    }
    Err(error::GetModInfoById::NotFound {
      id: id.as_ref().to_string(),
    })
  }
}
