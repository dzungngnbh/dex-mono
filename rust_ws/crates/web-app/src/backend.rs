use anyhow::Result;
use clickhouse::Client;
use shared::db::clickhouse::get_client;
use shared::db::redis as rediss;

use crate::lib::env::Env;

#[derive(Clone)]
pub struct Backend {
    pub ch_client: Client,
    pub redis_client: redis::Client,
}

impl Backend {
    pub fn new(env: &Env) -> Self {
        let ch_client = get_client(
            env.clickhouse_url.as_str(),
            env.clickhouse_pwd.as_str(),
            env.clickhouse_db.as_str(),
        );
        let redis_client = rediss::get_redis_client().unwrap();

        Self {
            ch_client,
            redis_client,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_backend() -> Result<()> {
        dotenvy::dotenv().ok();
        let env = Env::get_env()?;
        let backend = Backend::new(&env);
        Ok(())
    }
}
