use std::sync::Arc;

use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::mod_land::ModLand;

pub mod v1;

#[derive(OpenApi)]
#[openapi(info(title = "Korabli Mod Land", description = "Korabli Mod Land"))]
pub(crate) struct ApiDoc;

pub fn router() -> OpenApiRouter {
  OpenApiRouter::new().nest("/v1", v1::router())
}

#[derive(Clone)]
pub struct ModLandInstance(pub Arc<Mutex<ModLand>>);
