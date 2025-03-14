use std::{
  net::{IpAddr, SocketAddr},
  path::PathBuf,
  sync::Arc,
};

use axum::{Extension, Router};
use axum_client_ip::SecureClientIpSource;
use tokio::{fs, sync::Mutex};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
  api::{self, ApiDoc, ModLandInstance},
  middleware::AllowedIp,
  mod_land::{self, ModLand},
};

mod builder;
pub mod error;

pub struct Server {
  host: IpAddr,
  port: u16,
  api_mount: String,
  swagger_ui: bool,
  swagger_ui_mount: String,
  api_docs_mount: String,
  config_dir: PathBuf,
}

impl Default for Server {
  fn default() -> Self {
    Self {
      host: IpAddr::V4("127.0.0.1".parse().expect("never failed")),
      port: 6375,
      api_mount: "/api".to_string(),
      swagger_ui_mount: "/swagger-ui".to_string(),
      api_docs_mount: "/api-docs".to_string(),
      config_dir: PathBuf::from(".korabli-mod-land/config"),
      swagger_ui: true,
    }
  }
}

impl Server {
  pub fn builder() -> builder::Builder {
    builder::Builder::new()
  }

  fn api_router(&self) -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest(&self.api_mount, api::router())
  }

  async fn ensure_dirs(&mut self) -> Result<(), error::EnsureDirs> {
    // TODO
    Ok(())
  }

  async fn read_mod_land_config(
    &mut self,
  ) -> Result<mod_land::config::Config, error::ReadModLandConfig> {
    if let Ok(config) = fs::read(self.config_dir.join("mod_land.toml")).await {
      if let Ok(config) = toml::from_str::<mod_land::config::Config>(
        String::from_utf8_lossy(config.as_slice())
          .to_string()
          .as_str(),
      ) {
        Ok(config)
      } else {
        Ok(mod_land::config::Config::default())
      }
    } else {
      Ok(mod_land::config::Config::default())
    }
  }

  pub async fn serve(&mut self) -> Result<(), error::Serve> {
    self
      .ensure_dirs()
      .await
      .map_err(|err| error::Serve::EnsureDirs { source: err })?;

    let mod_land_config = self
      .read_mod_land_config()
      .await
      .map_err(|err| error::Serve::ReadModLandConfig { source: err })?;

    let (router, api) = self.api_router().split_for_parts();
    let router = Router::new().merge(router);
    let router = if self.swagger_ui {
      router.merge(
        SwaggerUi::new(self.swagger_ui_mount.to_owned())
          .url(format!("{}/openapi.json", self.api_docs_mount), api.clone()),
      )
    } else {
      router
    }
    .layer(Extension(ModLandInstance(Arc::new(Mutex::new(
      ModLand::try_from_config(mod_land_config)
        .map_err(|err| error::Serve::ModLandTryFromConfig { source: err })?,
    )))))
    .layer(SecureClientIpSource::RightmostXForwardedFor.into_extension())
    .layer(Extension(AllowedIp(vec![
      IpAddr::V4([127, 0, 0, 1].into()),
      IpAddr::V6("::1".parse().expect("not ipv6")),
    ])));

    let listener = tokio::net::TcpListener::bind((self.host, self.port))
      .await
      .expect("bind port failed");

    axum::serve(
      listener,
      router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("listening failed");

    Ok(())
  }

  pub fn openapi_json(&self) -> Result<String, ()> {
    let (_, api) = self.api_router().split_for_parts();
    Ok(api.to_pretty_json().expect("openapi_json failed"))
  }
}
