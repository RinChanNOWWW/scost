use opendal::Operator;
use prettytable::row;
use prettytable::table;
use prettytable::Table;

use super::Command;
use crate::bucket::BucketPtr;
use crate::Result;

pub struct List;

impl Command for List {
    const COMMAND: &'static str = "ls";
    const NUM_ARGS: usize = 2;
    const HELP: &'static str = "ls <bucket> <path>";

    async fn execute(buckets: &[BucketPtr], path: &str) -> Result<Table> {
        let mut tab = table!(["Bucket", "Object"]);

        let futs = buckets
            .iter()
            .map(|b| tokio::spawn(list_impl(b.op.clone(), path.to_owned())))
            .collect::<Vec<_>>();

        let results = futures::future::try_join_all(futs)
            .await?
            .into_iter()
            .collect::<Vec<_>>();

        for (b, res) in buckets.iter().zip(results.iter()) {
            match res {
                Ok(children) => {
                    children.iter().for_each(|path| {
                        tab.add_row(row![b.bucket.alias, path]);
                    });
                }
                Err(e) => {
                    tab.add_row(row![
                        b.bucket.alias,
                        format!("Failed to fetch objects: {}", e)
                    ]);
                }
            }
        }

        Ok(tab)
    }
}

async fn list_impl(op: Operator, path: String) -> Result<Vec<String>> {
    Ok(op
        .list(&path)
        .await?
        .iter()
        .map(|child| child.path().to_string())
        .collect())
}
