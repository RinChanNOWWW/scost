use crate::instance::Instance;
use crate::Config;
use crate::Result;

/// GLOBAL is a static type that holding all global data.
static GLOBAL: state::InitCell<Instance> = state::InitCell::new();

pub struct GlobalInstance;

impl GlobalInstance {
    pub fn init(config: &Config) -> Result<()> {
        let _ = GLOBAL.set(Instance::new(config)?);
        Ok(())
    }
    pub fn get() -> &'static Instance {
        GLOBAL.get()
    }
}
