pub mod cli;
pub mod handler;

use anyhow::{anyhow, Result};
use log::debug;
use reqwest::Client;
use smbpndk_model::{create_params, Oten};
use smbpndk_networking::{constants::BASE_URL, get_token};

async fn get_oten_apps() -> Result<Vec<Oten>> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/oten_apps"].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let oten_apps: Vec<Oten> = response.json().await?;
            Ok(oten_apps)
        }
        _ => Err(anyhow!("Failed to fetch oten apps.")),
    }
}

async fn get_oten_app(id: &str) -> Result<Oten> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/oten_apps/", id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let oten_app: Oten = response.json().await?;
            println!("Auth app requested: {oten_app:#?}");
            Ok(oten_app)
        }
        _ => Err(anyhow!(format!(
            "Failed to find an oten app with id: {id}."
        ))),
    }
}

async fn delete_oten_app(id: String) -> Result<()> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .delete([BASE_URL, "v1/oten_apps/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("Project deleted.");
            Ok(())
        }
        _ => Err(anyhow!("Failed to delete an oten app.")),
    }
}

async fn create_oten_app(oten_app: create_params::Oten) -> Result<Oten> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .post([BASE_URL, "v1/oten_apps"].join(""))
        .json(&oten_app)
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let oten_app: Oten = response.json().await?;
            println!("Project created: {oten_app:#?}");
            Ok(oten_app)
        }
        _ => Err(anyhow!("Failed to create an oten app.")),
    }
}
