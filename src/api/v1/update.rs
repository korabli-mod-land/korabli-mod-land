use axum::{
  Extension,
  body::Body,
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};

use crate::{api::ModLandInstance, mod_land};

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
  #[error("ModLand::FetchRepository: {source}")]
  ModLandFetchRespotirory {
    source: mod_land::error::FetchRepository,
  },
  #[error("Unknown Repository: repo: {repo}")]
  UnknownRepository { repo: String },
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    match self {
      Self::ModLandFetchRespotirory { .. } => {
        (StatusCode::INTERNAL_SERVER_ERROR, Body::empty()).into_response()
      }
      Self::UnknownRepository { repo } => (
        StatusCode::BAD_REQUEST,
        format!("Repository '{}' not exists", repo),
      )
        .into_response(),
    }
  }
}

#[utoipa::path(
  post,
  path = "/update/{repo}",
  params(
    ("repo", description = "Repository to update.")
  ),
  operation_id = "v1_update",
  tag = "lifetime cycle",
  responses(
    (status = StatusCode::OK, description = "Update success"),
    (status = StatusCode::BAD_REQUEST, description = "Unknown repository"),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Server error", body = String),
  )
)]
pub(super) async fn update(
  Path(repo): Path<String>,
  Extension(mod_land): Extension<ModLandInstance>,
) -> Result<(), Error> {
  let mut mod_land = mod_land.0.lock().await;
  let fetched = mod_land
    .fetch_repository(&repo)
    .await
    .map_err(|err| Error::ModLandFetchRespotirory { source: err })?;
  if !fetched {
    Err(Error::UnknownRepository { repo })
  } else {
    Ok(())
  }
}
