use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
#[serde(default)]
pub struct Args {
    #[clap(long, short = 'c', required = true)]
    pub config_file: String,
}
