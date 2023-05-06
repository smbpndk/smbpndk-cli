pub mod cli;
pub mod handler;

use anyhow::{anyhow, Result};
use log::debug;
use reqwest::Client;
use smbpndk_model::{AppCreate, Rdb};
use smbpndk_networking::{constants::BASE_URL, get_token};

async fn get_rdb_apps() -> Result<Vec<Rdb>> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/rdb_apps"].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let apps: Vec<Rdb> = response.json().await?;
            Ok(apps)
        }
        _ => Err(anyhow!("Failed to fetch Rdb apps.")),
    }
}

async fn get_rdb_app(id: &str) -> Result<Rdb> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/rdb_apps/", id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let app: Rdb = response.json().await?;
            println!("Rdb app requested: {app:#?}");
            Ok(app)
        }
        _ => Err(anyhow!(format!("Failed to find an Rdb app with id: {id}."))),
    }
}

async fn delete_rdb_app(id: String) -> Result<()> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .delete([BASE_URL, "v1/rdb_apps/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("Project deleted.");
            Ok(())
        }
        _ => Err(anyhow!("Failed to delete an Rdb app.")),
    }
}

async fn create_rdb_app(app: AppCreate) -> Result<Rdb> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .post([BASE_URL, "v1/rdb_apps"].join(""))
        .json(&app)
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let app: Rdb = response.json().await?;
            println!("Project created: {app:#?}");
            Ok(app)
        }
        _ => Err(anyhow!("Failed to create an Rdb app.")),
    }
}
