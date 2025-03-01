use opendal::raw::normalize_path;
use prettytable::row;
use prettytable::table;
use prettytable::Table;

use super::Command;
use crate::bucket::BucketPtr;
use crate::Result;

pub struct Copy;

impl Command for Copy {
    const COMMAND: &'static str = "cp";
    const NUM_ARGS: usize = 3;
    const HELP: &'static str = "cp <from_bucket> <to_bucket> <path>";

    async fn execute(buckets: &[BucketPtr], path: &str) -> Result<Table> {
        let mut tab = table!(["Bucket", "Result"]);

        let from_bucket = buckets[0].clone();
        let to_buckets = buckets[1..].to_vec();

        let futs = to_buckets
            .iter()
            .map(|b| tokio::spawn(copy_impl(from_bucket.clone(), b.clone(), path.to_owned())))
            .collect::<Vec<_>>();

        let results = futures::future::try_join_all(futs)
            .await?
            .into_iter()
            .collect::<Vec<_>>();

        for (b, res) in buckets.iter().zip(results.iter()) {
            if let Err(e) = res {
                tab.add_row(row![b.bucket.alias, format!("Failed: {}", e)]);
            } else {
                tab.add_row(row![b.bucket.alias, "OK"]);
            }
        }

        Ok(tab)
    }
}

async fn copy_impl(_: BucketPtr, _: BucketPtr, path: String) -> Result<()> {
    // OpenDAL doesn't support copying object across buckets now.
    // Need to implement it by myself.
    let _ = normalize_path(&path);
    todo!("Impl copy")
}
