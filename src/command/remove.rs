use opendal::Operator;
use prettytable::row;
use prettytable::table;
use prettytable::Table;

use super::Command;
use crate::bucket::BucketPtr;
use crate::Result;

pub struct Remove;

impl Command for Remove {
    const COMMAND: &'static str = "rm";
    const NUM_ARGS: usize = 2;
    const HELP: &'static str = "rm <bucket> <path>";

    async fn execute(buckets: &[BucketPtr], path: &str) -> Result<Table> {
        let mut tab = table!(["Bucket", "Result"]);

        let futs = buckets
            .iter()
            .map(|b| tokio::spawn(remove_impl(b.op.clone(), path.to_owned())))
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

async fn remove_impl(op: Operator, path: String) -> Result<()> {
    op.remove_all(&path).await?;
    Ok(())
}
