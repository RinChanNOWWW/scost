use chrono::Utc;
use http::header::CONTENT_LENGTH;
use http::header::DATE;
use http::header::HOST;
use http::Request;
use opendal::raw::build_abs_path;
use opendal::raw::format_datetime_into_http_date;
use opendal::raw::new_request_build_error;
use opendal::raw::normalize_path;
use opendal::raw::percent_encode_path;
use opendal::raw::HttpClient;
use opendal::Buffer;
use prettytable::row;
use prettytable::table;
use prettytable::Table;

use super::Command;
use crate::bucket::BucketPtr;
use crate::GlobalInstance;
use crate::Result;

pub struct Copy;

impl Command for Copy {
    const COMMAND: &'static str = "cp";
    const NUM_ARGS: usize = 3;
    const HELP: &'static str = "cp <from_bucket> <to_bucket> <path>";

    async fn execute(buckets: &[BucketPtr], path: &str) -> Result<Table> {
        let mut tab = table!(["Source", "Target", "Result"]);

        let from_bucket = buckets[0].clone();
        let to_buckets = buckets[1..].to_vec();

        let client = HttpClient::new()?;

        let futs = to_buckets
            .iter()
            .filter(|b| b.bucket.alias != from_bucket.bucket.alias)
            .map(|b| {
                tokio::spawn(copy_impl(
                    client.clone(),
                    from_bucket.clone(),
                    b.clone(),
                    path.to_owned(),
                ))
            })
            .collect::<Vec<_>>();

        let results = futures::future::try_join_all(futs)
            .await?
            .into_iter()
            .collect::<Vec<_>>();

        for (b, res) in to_buckets.iter().zip(results.iter()) {
            if let Err(e) = res {
                tab.add_row(row![
                    from_bucket.bucket.alias,
                    b.bucket.alias,
                    format!("Failed: {}", e)
                ]);
            } else {
                tab.add_row(row![from_bucket.bucket.alias, b.bucket.alias, "OK"]);
            }
        }

        Ok(tab)
    }
}

async fn copy_impl(
    client: HttpClient,
    source_bucket: BucketPtr,
    target_bucket: BucketPtr,
    path: String,
) -> Result<()> {
    // OpenDAL doesn't support copying object across buckets now.
    // Need to implement it by myself.
    let path = normalize_path(&path);

    let source_info = source_bucket.op.info();
    let target_info = target_bucket.op.info();

    let source = build_abs_path(source_info.root(), &path);
    let target = build_abs_path(target_info.root(), &path);

    let mut req = Request::put(format!(
        "https://cos.{}.myqcloud.com/{}",
        target_bucket.bucket.region,
        percent_encode_path(&target)
    ))
    .header(DATE, format_datetime_into_http_date(Utc::now()))
    .header(
        HOST,
        format!(
            "{}.cos.{}.myqcloud.com",
            target_bucket.bucket.bucket, target_bucket.bucket.region
        ),
    )
    .header(
        "x-cos-copy-source",
        format!(
            "{}.cos.{}.myqcloud.com/{}",
            source_bucket.bucket.bucket,
            source_bucket.bucket.region,
            percent_encode_path(&source)
        ),
    )
    .header(CONTENT_LENGTH, 0)
    .body(Buffer::new())
    .map_err(new_request_build_error)?;

    GlobalInstance::instance().sign_request(&mut req)?;

    client.send(req).await?;

    Ok(())
}
