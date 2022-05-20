use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(rename = "target")]
    pub(crate) targets: HashMap<String, Target>,
    #[serde(rename = "build-path", default = "default_build_path")]
    pub(crate) build_path: String,
    #[serde(rename = "default-bin", default)]
    pub(crate) default_bin: String,
    #[serde(rename = "default-target", default)]
    pub(crate) default_target: String,
}

fn default_build_path() -> String {
    "build".into()
}

#[derive(Deserialize)]
pub(crate) struct Target {
    pub(crate) args: Vec<String>,
    #[serde(rename = "build")]
    pub(crate) build_command: String,
}
