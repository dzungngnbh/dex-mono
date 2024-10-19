use anyhow::Result;
use std::env;

pub fn get_private_key() -> Result<String> {
    dotenvy::dotenv().ok();
    Ok(env::var("PRIVATE_KEY")?)
}
