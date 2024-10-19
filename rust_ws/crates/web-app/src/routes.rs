use anyhow::Result;

use crate::lib;

pub fn host_name() -> Result<String> {
    let env = lib::env::Env::get_env()?;
    if env.env == "prod" {
        return Ok("tradingexec.xyz".to_string());
    }

    Ok("localhost:3000".to_string())
}
