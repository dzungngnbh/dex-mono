use anyhow::Result;
use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct Env {
    pub env: String,
    pub clickhouse_url: String,
    pub clickhouse_pwd: String,
    pub clickhouse_db: String,
}

static VALID_ENVS: [&str; 3] = ["dev", "prod", "staging"];

impl Env {
    pub fn get_env() -> Result<Self> {
        dotenv()?;
        let env = Env {
            env: env::var("ENV")?,
            clickhouse_url: env::var("CLICKHOUSE_URL")?,
            clickhouse_pwd: env::var("CLICKHOUSE_PWD")?,
            clickhouse_db: env::var("CLICKHOUSE_DB")?,
        };

        println!("env: {:?}", env);
        env.validate()?;
        Ok(env)
    }

    pub fn validate(&self) -> Result<()> {
        if (VALID_ENVS.contains(&self.env.as_str())
            && !self.clickhouse_db.is_empty()
            && !self.clickhouse_url.is_empty())
        // && !self.clickhouse_pwd.is_empty() // pwd can be empty
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
    fn test_get_env() -> Result<()> {
        let env = Env::get_env()?;
        Ok(())
    }
}
