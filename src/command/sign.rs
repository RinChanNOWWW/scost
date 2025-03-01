use std::time::Duration;

use opendal::Operator;
use prettytable::row;
use prettytable::table;
use prettytable::Table;

use super::Command;
use crate::bucket::BucketPtr;
use crate::Result;

pub struct Sign;

impl Command for Sign {
    const COMMAND: &'static str = "sign";
    const NUM_ARGS: usize = 2;
    const HELP: &'static str = "sign <bucket> <path>";

    async fn execute(buckets: &[BucketPtr], path: &str) -> Result<Table> {
        let mut tab = table!(["Bucket", "Result"]);

        let futs = buckets
            .iter()
            .map(|b| tokio::spawn(sign_impl(b.op.clone(), path.to_owned())))
            .collect::<Vec<_>>();

        let results = futures::future::try_join_all(futs)
            .await?
            .into_iter()
            .collect::<Vec<_>>();

        for (b, res) in buckets.iter().zip(results.iter()) {
            match res {
                Ok(s) => {
                    tab.add_row(row![b.bucket.alias, s]);
                }
                Err(e) => {
                    tab.add_row(row![b.bucket.alias, format!("Failed: {}", e)]);
                }
            }
        }

        Ok(tab)
    }
}

async fn sign_impl(op: Operator, path: String) -> Result<String> {
    let res = op.presign_read(&path, Duration::from_hours(2)).await?;
    Ok(res.uri().to_string())
}
