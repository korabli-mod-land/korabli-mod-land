use utoipa_axum::{router::OpenApiRouter, routes};

mod info;

pub(super) fn router() -> OpenApiRouter {
  OpenApiRouter::new().routes(routes!(info::info))
}
