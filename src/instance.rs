use std::collections::HashMap;

use anyhow::Error;
use http::Request;
use opendal::raw::new_request_sign_error;
use reqsign::TencentCosCredential;
use reqsign::TencentCosSigner;

use crate::bucket::Bucket;
use crate::bucket::BucketPtr;
use crate::config::Auth;
use crate::Config;
use crate::Result;

pub struct Instance {
    /// Map alias -> OpenDAL operator
    handles: HashMap<String, BucketPtr>,
    signer: TencentCosSigner,
    cred: TencentCosCredential,
}

impl From<Auth> for TencentCosCredential {
    fn from(auth: Auth) -> Self {
        Self {
            secret_id: auth.secret_id,
            secret_key: auth.secret_key,
            security_token: None,
            expires_in: None,
        }
    }
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

        Ok(Self {
            handles,
            signer: TencentCosSigner::new(),
            cred: TencentCosCredential::from(config.auth.clone()),
        })
    }

    pub fn get_bucket(&self, alias: &str) -> Option<BucketPtr> {
        self.handles.get(alias).cloned()
    }

    pub fn get_buckets(&self) -> Vec<BucketPtr> {
        self.handles.values().cloned().collect()
    }

    pub fn sign_request<T>(&self, req: &mut Request<T>) -> Result<()> {
        self.signer
            .sign(req, &self.cred)
            .map_err(new_request_sign_error)?;
        Ok(())
    }
}
