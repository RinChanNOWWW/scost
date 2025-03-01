mod bucket;
#[allow(async_fn_in_trait)]
pub mod command;
#[allow(clippy::module_inception)]
mod config;
mod global;
mod instance;
mod interpreter;

pub use config::Args;
pub use config::Config;
pub use global::GlobalInstance;
pub use interpreter::Interpreter;

pub type Result<T> = anyhow::Result<T>;
