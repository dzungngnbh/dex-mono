use anyhow::{bail, Result};
use futures::StreamExt;
use redis::{AsyncCommands, Client, Commands};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Data not found")]
    NotFound,
}

fn get_client<S: Into<String>>(conn_str: S) -> Result<Client> {
    let conn_str = conn_str.into();
    Ok(Client::open(conn_str)?)
}

pub async fn set(client: &Client, k: &str, bin_data: &Vec<u8>) -> Result<()> {
    let mut conn = client.get_async_connection().await?;
    redis::cmd("SET")
        .arg(k)
        .arg(bin_data)
        .query_async(&mut conn)
        .await?;
    Ok(())
}

// returns raw data in vec<u8> instead of casting to T with bincode
// you can bring your own deserialization here
pub async fn get_raw(client: &Client, k: &str) -> Result<Vec<u8>> {
    let mut conn = client.get_async_connection().await?;
    let bin_data: Vec<u8> = match redis::cmd("GET").arg(k).query_async(&mut conn).await {
        Ok(bin_data) => bin_data,
        Err(e) => {
            if e.is_connection_dropped() {
                bail!(ErrorKind::NotFound);
            } else {
                bail!(e);
            }
        }
    };

    if bin_data.is_empty() {
        bail!(ErrorKind::NotFound);
    }

    Ok(bin_data)
}

pub async fn get<T>(client: &Client, k: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let bin_data = get_raw(client, k).await?;
    let res: T = bincode::deserialize(&bin_data)?;
    Ok(res)
}

pub async fn gets<T>(client: &Client, k_pattern: &str) -> Result<Vec<Result<T>>>
where
    T: DeserializeOwned,
{
    let mut conn = client.get_async_connection().await?;
    let iter = conn.scan_match::<&str, String>(k_pattern).await?;
    let ks: Vec<String> = iter.collect().await;

    let result: Vec<Vec<u8>> = redis::cmd("MGET").arg(&ks).query_async(&mut conn).await?;
    let res = result
        .iter()
        .map(|v| {
            let res: T = bincode::deserialize(v)?;
            Ok(res)
        })
        .collect::<Vec<Result<T>>>();

    Ok(res)
}

// This can be used to get dragonfly client as well
pub fn get_redis_client() -> Result<Client> {
    dotenvy::dotenv().ok();
    let conn_str = std::env::var("REDIS_CONN_STR").unwrap();
    let client = get_client(conn_str)?;
    Ok(client)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    #[ignore]
    fn test_get_client() -> Result<()> {
        let client = get_client("redis://localhost:6379")?;
        let mut conn = client.get_connection()?;

        let time_start = std::time::Instant::now();
        let _: () = conn.set("foo", "bar")?;
        let res: String = conn.get("foo")?;
        let time_end = std::time::Instant::now();
        let duration = time_end - time_start;
        println!("Duration: {:?}", duration);

        dbg!(res);
        Ok(())
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct Test {
        pub a: String,
    }

    #[tokio::test]
    #[ignore]
    async fn test_insert_data() -> Result<()> {
        dotenvy::dotenv().ok();
        let client = get_redis_client()?;

        let test = Test {
            a: String::from("test"),
        };

        set(&client, "test", &bincode::serialize(&test)?).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_data_success() -> Result<()> {
        let client = get_redis_client()?;
        let mut conn = client.get_async_connection().await?;
        let k = "test";
        let get_res: Test = get(&client, k).await?;
        // let k = cron_strategy_command_details_k(&uuid!("3f699d13-9c5e-4383-994c-3283d39b4c9a")); // for debugging and testing
        // let get_res: TimeBasedCronDetails = get(&client, k.as_str()).await?;
        dbg!(get_res);

        Ok(())
    }

    async fn get_non_existed_data(client: &Client) -> Result<()> {
        let get_res: Test = get(client, "not-existed").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_data_failed() -> Result<()> {
        let client = get_redis_client()?;
        let res = get_non_existed_data(&client).await;
        match res {
            Ok(_) => panic!("Should not be Ok"),
            Err(e) => match e.downcast_ref::<ErrorKind>() {
                Some(ErrorKind::NotFound) => (),
                _ => panic!("Should be NotFound"),
            },
        }

        Ok(())
    }
}
