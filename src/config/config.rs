use env_home::env_home_dir;
use serde::Deserialize;
use serde::Serialize;

use crate::Result;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Auth {
    pub secret_id: String,
    pub secret_key: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Bucket {
    pub alias: String,
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub auth: Auth,
    pub buckets: Vec<Bucket>,
}

impl Config {
    pub fn load(file: &str) -> Result<Self> {
        let c = if file.is_empty() {
            serfig::collectors::from_file(
                serfig::parsers::Toml,
                env_home_dir()
                    .unwrap_or("".into())
                    .join(".scost.toml")
                    .to_str()
                    .unwrap_or(""),
            )
        } else {
            serfig::collectors::from_file(serfig::parsers::Toml, file)
        };
        serfig::Builder::default().collect(c).build()
    }
}
