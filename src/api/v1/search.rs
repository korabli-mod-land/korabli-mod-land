use std::sync::Arc;

use axum::{Extension, Json, extract::Query, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{api::ModLandInstance, mod_land};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
  #[error("ModLand::SearchMod: {source}")]
  ModLandSearchMod {
    source: Arc<mod_land::error::SearchMod>,
  },
  #[error("NoSearchPatternProvided")]
  NoSearchPatternProvided,
}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::ModLandSearchMod { .. } => (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response(),
      Self::NoSearchPatternProvided => {
        (StatusCode::BAD_REQUEST, "No Search Pattern Provided").into_response()
      }
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
  id: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SearchRes {
  matches: Vec<String>,
}

impl IntoResponse for SearchRes {
  fn into_response(self) -> axum::response::Response {
    (StatusCode::OK, Json(self)).into_response()
  }
}

#[utoipa::path(
  get,
  path = "/search",
  operation_id = "v1_search",
  tag = "search",
  params(
    (
      "id" = Option<String>,
      Query,
      description = "Mod ID",
    ),
  ),
  responses(
    (status = StatusCode::OK, description = "Search completed.", body = SearchRes),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Server Error."),
    (status = StatusCode::BAD_REQUEST, description = "Bad request.", body = String),
  )
)]
pub(super) async fn search(
  Query(SearchQuery { id }): Query<SearchQuery>,
  Extension(mod_land): Extension<ModLandInstance>,
) -> Result<SearchRes, Error> {
  let mut mod_land = mod_land.0.lock().await;
  if let Some(id) = id {
    let matches = mod_land
      .search_mod_by_id(id)
      .await
      .map_err(|err| Error::ModLandSearchMod {
        source: Arc::new(err),
      })?
      .into_iter()
      .map(|(_, y)| y)
      .collect();
    Ok(SearchRes { matches })
  } else {
    Err(Error::NoSearchPatternProvided)
  }
}
