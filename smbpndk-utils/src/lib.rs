use std::{fs::OpenOptions, io::Write};

use anyhow::{anyhow, Result};
use log::debug;
use regex::Regex;
use smbpndk_model::Config;

pub fn email_validation(input: &str) -> Result<(), &'static str> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    if email_regex.is_match(input) {
        Ok(())
    } else {
        Err("Username must be an email address.")
    }
}

pub fn write_config(config: Config) -> Result<Config> {
    match home::home_dir() {
        Some(path) => {
            debug!("{}", path.to_str().unwrap());
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open([path.to_str().unwrap(), "/.smb/config.json"].join(""))?;
            let json = serde_json::to_string(&config)?;
            file.write_all(json.as_bytes())?;

            Ok(config)
        }
        None => Err(anyhow!("Error getting config.")),
    }
}
