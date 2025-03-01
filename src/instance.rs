use std::collections::HashMap;

use anyhow::Error;

use crate::bucket::Bucket;
use crate::bucket::BucketPtr;
use crate::Config;
use crate::Result;

pub struct Instance {
    /// Map alias -> OpenDAL operator
    handles: HashMap<String, BucketPtr>,
}

impl Instance {
    pub fn new(config: &Config) -> Result<Self> {
        let mut handles = HashMap::with_capacity(config.buckets.len());
        for b in &config.buckets {
            if b.alias == "*" {
                return Err(Error::msg("'*' is a reserved alias for all buckets"));
            }
            let bucket = Bucket::new(b, &config.auth)?;
            handles.insert(b.alias.clone(), bucket);
        }
        Ok(Self { handles })
    }

    pub fn get_bucket(&self, alias: &str) -> Option<BucketPtr> {
        self.handles.get(alias).cloned()
    }

    pub fn get_buckets(&self) -> Vec<BucketPtr> {
        self.handles.values().cloned().collect()
    }
}
