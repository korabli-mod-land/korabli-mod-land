use std::net::IpAddr;

use clap::Parser;
use mod_land::mod_info::ModInfo;
use schemars::schema_for;
use tokio::fs;

mod api;
mod cli;
mod middleware;
mod mod_land;
mod server;

#[tokio::main]
async fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

  let cli = cli::Cli::parse();

  match cli.command {
    cli::Commands::Serve { host, port } => {
      let mut server = server::Server::builder();

      if let Some(host) = host {
        server.host(IpAddr::V4(host.parse().expect("host incorrect")));
      }

      if let Some(port) = port {
        server.port(port);
      }

      let mut server = server.done();

      server.serve().await.expect("server failed")
    }
    cli::Commands::Generate { item, output } => {
      let server = server::Server::builder().done();
      fs::write(
        output,
        match item.as_str() {
          "openapi" => server
            .openapi_json()
            .expect("openapi json failed")
            .as_bytes()
            .to_vec(),
          "mod-info-schema" => {
            serde_json::to_vec_pretty(&schema_for!(ModInfo)).expect("mod-info-schema json failed")
          }
          _ => {
            panic!("unknown generate item: {}", item);
          }
        },
      )
      .await
      .expect("write failed");
    }
  }
}
