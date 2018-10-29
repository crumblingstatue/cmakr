use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
crate struct Config {
    #[serde(rename = "target")]
    crate targets: HashMap<String, Target>,
    #[serde(rename = "build-path", default = "default_build_path")]
    crate build_path: String,
}

fn default_build_path() -> String {
    "build".into()
}

#[derive(Deserialize)]
crate struct Target {
    crate args: Vec<String>,
    #[serde(rename = "build")]
    crate build_command: String,
}
