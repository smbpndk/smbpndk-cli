pub mod cli;
pub mod handler;

use anyhow::{anyhow, Result};
use log::debug;
use reqwest::Client;
use smbpndk_model::{AppCreate, Pkt};
use smbpndk_networking::{constants::BASE_URL, get_token};

async fn get_pkt_apps() -> Result<Vec<Pkt>> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/pkt_apps"].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let apps: Vec<Pkt> = response.json().await?;
            Ok(apps)
        }
        _ => Err(anyhow!("Failed to fetch Pkt apps.")),
    }
}

async fn get_pkt_app(id: &str) -> Result<Pkt> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/pkt_apps/", id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let auth_app: Pkt = response.json().await?;
            println!("Pkt app requested: {auth_app:#?}");
            Ok(auth_app)
        }
        _ => Err(anyhow!(format!("Failed to find an Pkt app with id: {id}."))),
    }
}

async fn delete_pkt_app(id: String) -> Result<()> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .delete([BASE_URL, "v1/pkt_apps/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("App deleted.");
            Ok(())
        }
        _ => Err(anyhow!("Failed to delete an Pkt app.")),
    }
}

async fn create_pkt_app(app: AppCreate) -> Result<Pkt> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .post([BASE_URL, "v1/pkt_apps"].join(""))
        .json(&app)
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let app: Pkt = response.json().await?;
            println!("App created: {app:#?}");
            Ok(app)
        }
        _ => Err(anyhow!("Failed to create an Pkt app.")),
    }
}
