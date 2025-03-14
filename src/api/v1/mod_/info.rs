use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};

use crate::{
  api::ModLandInstance,
  mod_land::{error::GetModInfoById, mod_info::ModInfo},
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
  #[error("GetModInfoById: {source}")]
  GetModInfoById { source: Arc<GetModInfoById> },
}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::GetModInfoById { source } => match *source {
        GetModInfoById::NotFound { ref id } => {
          (StatusCode::NOT_FOUND, format!("Mod '{id}' not found")).into_response()
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response(),
      },
    }
  }
}

#[utoipa::path(
  get,
  path = "/info/{id}",
  params(
    (
      "id" = String,
      description = "Mod ID",
    )
  ),
  operation_id = "v1_info",
  tag = "mod",
  responses(
    (status = StatusCode::OK, description = "Mod info got", body = ModInfo),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Server error"),
    (status = StatusCode::NOT_FOUND, description = "Mod not found", body = String),
  )
)]
pub(super) async fn info(
  Path(id): Path<String>,
  Extension(mod_land): Extension<ModLandInstance>,
) -> Result<Json<ModInfo>, Error> {
  let mut mod_land = mod_land.0.lock().await;
  mod_land
    .get_mod_info_by_id(id)
    .await
    .map(|mod_info| Json(mod_info))
    .map_err(|err| Error::GetModInfoById {
      source: Arc::new(err),
    })
}
