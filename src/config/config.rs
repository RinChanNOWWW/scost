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
        serfig::Builder::default()
            .collect(serfig::collectors::from_file(serfig::parsers::Toml, file))
            .build()
    }
}
