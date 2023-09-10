pub mod constants;

use std::env;

use anyhow::{anyhow, Result};
use log::debug;
use url_builder::URLBuilder;

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

pub fn smb_base_url_builder() -> URLBuilder {
    let client_id = env::var("SMB_CLIENT_ID").expect("Please set SMB_CLIENT_ID");
    let client_secret = env::var("SMB_CLIENT_SECRET").expect("Please set SMB_CLIENT_SECRET");
    let api_host = env::var("SMB_API_HOST").expect("Please set SMB_API_HOST");
    let api_protocol = env::var("SMB_API_PROTOCOL").expect("Please set SMB_API_PROTOCOL");
    let mut url_builder = URLBuilder::new();
    url_builder
        .set_protocol(&api_protocol)
        .set_host(&api_host)
        .add_param("client_id", &client_id)
        .add_param("client_secret", &client_secret);
    url_builder
}
