use anyhow::Result;
use std::env;

#[derive(Debug)]
pub struct Env {
    pub env: String,
    pub port: u16, // Add this new field
    pub clickhouse_url: String,
    pub clickhouse_pwd: String,
    pub clickhouse_db: String,
    pub redis_conn_str: String,
}

static VALID_ENVS: [&str; 3] = ["dev", "prod", "staging"];

const DEFAULT_PORT: u16 = 3000;

impl Env {
    pub fn get_env() -> Result<Self> {
        let env = Env {
            env: env::var("ENV")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(DEFAULT_PORT),
            clickhouse_url: env::var("CLICKHOUSE_URL")?,
            clickhouse_pwd: env::var("CLICKHOUSE_PWD")?,
            clickhouse_db: env::var("CLICKHOUSE_DB")?,
            redis_conn_str: env::var("REDIS_CONN_STR")?,
        };

        env.validate()?;
        Ok(env)
    }

    pub fn validate(&self) -> Result<()> {
        if VALID_ENVS.contains(&self.env.as_str())
            && !self.clickhouse_db.is_empty()
            && !self.clickhouse_url.is_empty()
            // && !self.clickhouse_pwd.is_empty() // pwd can be empty
            && !self.redis_conn_str.is_empty()
            && self.port > 0
        {
            return Ok(());
        }

        panic!("Invalid ENV variables");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env() {
        let env = Env::get_env().unwrap();
    }
}
