use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct ModInfo {
  pub id: String,
  pub name: String,
  pub description: String,
  pub versions: HashMap<String, VersionInfo>,
  pub authors: Vec<String>,
  pub dependencies: Option<HashMap<String, String>>,
  #[serde(rename = "type", default)]
  pub ty: ModType,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ModType {
  #[default]
  Game,
  Meta,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct VersionInfo {
  pub url: String,
  pub sha512sum: String,
}
