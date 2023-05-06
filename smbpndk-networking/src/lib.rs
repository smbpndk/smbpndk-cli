pub mod constants;

use anyhow::{anyhow, Result};
use log::debug;
use smbpndk_model::Config;

pub async fn get_token() -> Result<String> {
    if let Some(mut path) = dirs::home_dir() {
        path.push(".smb/token");
        std::fs::read_to_string(path).map_err(|e| {
            debug!("Error while reading token: {}", &e);
            anyhow!("Error while reading token. Are you logged in?")
        })
    } else {
        Err(anyhow!("Failed to get home directory."))
    }
}

pub async fn get_config() -> Result<Config> {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".smb/config");
            let config = std::fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&config)?;
            Ok(config)
        }
        None => Err(anyhow!("Failed to get home directory.")),
    }
}
