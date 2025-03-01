use std::process::exit;

use clap::Parser;
use scost::Args;
use scost::Config;
use scost::GlobalInstance;
use scost::Interpreter;
use scost::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load(&args.config_file)?;
    GlobalInstance::init(&config)?;

    let mut interpreter = Interpreter::new()?;
    if let Err(e) = interpreter.run().await {
        eprintln!("{}", e);
        exit(1)
    }

    Ok(())
}
