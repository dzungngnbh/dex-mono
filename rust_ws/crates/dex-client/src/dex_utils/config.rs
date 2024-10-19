use anyhow::Result;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentConfig {
    pub node_url: String,
    pub explorer_url: String,
    pub start_block: u64,
    pub quote: String,
    pub fee_calculator: String,
    pub clearinghouse: String,
    pub endpoint: String,
    pub spot_engine: String,
    pub perp_engine: String,
    pub arb_airdrop: String,
    pub chain_id: u64,
}

impl DeploymentConfig {
    pub fn new(config_mode: ConfigMode) -> Result<Self> {
        let config_path = match config_mode {
            ConfigMode::Local => ".env_config_local.json",
        };
        let config = std::fs::read_to_string(config_path)?;
        let config: DeploymentConfig = serde_json::from_str(&config)?;
        Ok(config)
    }
}

#[derive(Clone)]
pub enum ConfigMode {
    Local,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_config_new() -> Result<()> {
        let config = DeploymentConfig::new(ConfigMode::Local).unwrap();
        assert_eq!(config.node_url, "http://127.0.0.1:8545");
        Ok(())
    }
}
