use anyhow::Result;
use clickhouse::Client;
use shared::db::clickhouse::get_client;

use crate::lib::env::{get_env, Env};

#[derive(Clone)]
pub struct Backend {
    pub ch_client: Client,
}

impl Backend {
    pub fn new(env: &Env) -> Self {
        let ch_client = get_client(
            env.clickhouse_url.as_str(),
            env.clickhouse_pwd.as_str(),
            env.clickhouse_db.as_str(),
        );

        Self { ch_client }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_backend() -> Result<()> {
        let env = Env::get_env()?;
        let backend = Backend::new(&env);
        Ok(())
    }
}
