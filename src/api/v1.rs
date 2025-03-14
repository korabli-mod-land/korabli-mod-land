use axum::middleware;
use utoipa_axum::{
  router::{OpenApiRouter, UtoipaMethodRouterExt},
  routes,
};

use crate::middleware::validate_client_ip;

mod hello;
mod mod_;
mod search;
mod update;

pub(super) fn router() -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(hello::hello))
    .routes(routes!(update::update).layer(middleware::from_fn(validate_client_ip)))
    .routes(routes!(search::search))
    .nest("/mod", mod_::router())
}
