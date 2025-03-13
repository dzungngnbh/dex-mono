use anyhow::Result;

use shared::db::redis as rediss;

#[cfg(test)]
fn get_redis_client() -> Result<redis::Client> {
    Ok(rediss::get_redis_client()?)
}
