use anyhow::Result;
use clickhouse::Client;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;
use uuid::Uuid;

pub fn get_client(url: &str, pwd: &str, db_name: &str) -> Client {
    const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(2);

    let https = HttpsConnector::new();
    let http_client = hyper::Client::builder()
        .pool_idle_timeout(POOL_IDLE_TIMEOUT)
        .build::<_, hyper::Body>(https);

    let mut client = Client::with_http_client(http_client)
        .with_url(url)
        .with_user("default")
        .with_password(pwd);

    if !db_name.is_empty() {
        client = client.with_database(db_name);
    }

    client
}

/// Generic insert function for clickhouse, it can insert a vector of records.
///
pub async fn insert<T>(
    client: &clickhouse::Client,
    table_name: &str,
    records: &Vec<T>,
) -> Result<()>
where
    T: clickhouse::Row + Serialize,
{
    if records.is_empty() {
        return Ok(());
    }

    let mut insert = client.insert(table_name)?;
    for record in records {
        insert.write(record).await?;
    }
    insert.end().await?;
    Ok(())
}

pub async fn get<T>(client: &clickhouse::Client, table: &str, id: &Uuid) -> Result<T>
where
    T: clickhouse::Row + DeserializeOwned,
{
    let query = format!(
        "SELECT * EXCEPT(sign) FROM {} \
                 WHERE id = ? \
                 GROUP BY * EXCEPT(sign) HAVING sum(sign) > 0",
        table
    );
    Ok(client
        .query(query.as_str())
        .bind(id)
        .fetch_one::<T>()
        .await?)
}

/// General update, since it needs to use VersionedCollapsingMergeTree, we need to cancel current one, and insert new version.

#[cfg(test)]
mod tests {
    use super::*;

    use clickhouse::Row;
    use serde::Deserialize;

    #[derive(Row, Deserialize, Debug)]
    struct CHService<'a> {
        id: u32,
        value: &'a str,
        label: &'a str,
    }
}
