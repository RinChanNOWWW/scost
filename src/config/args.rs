use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
#[serde(default)]
pub struct Args {
    #[clap(long, short = 'c', default_value = "")]
    pub config_file: String,
}
