use std::sync::Arc;

use opendal::services;
use opendal::Operator;

use crate::config;
use crate::Result;

pub struct Bucket {
    pub bucket: config::Bucket,
    pub op: Operator,
}

impl Bucket {
    pub fn new(bucket: &config::Bucket, auth: &config::Auth) -> Result<BucketPtr> {
        let op = Operator::new(
            services::Cos::default()
                .endpoint(&format!("https://cos.{}.myqcloud.com", bucket.region))
                .secret_id(&auth.secret_id)
                .secret_key(&auth.secret_key)
                .bucket(&bucket.bucket),
        )?
        .finish();
        Ok(Arc::new(Bucket {
            bucket: bucket.clone(),
            op,
        }))
    }
}

pub type BucketPtr = Arc<Bucket>;
