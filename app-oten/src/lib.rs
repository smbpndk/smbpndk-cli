use anyhow::{anyhow, Result};
use log::debug;
use reqwest::Client;
use smbpndk_model::{AuthApp, AuthAppCreate};
use smbpndk_networking::{constants::BASE_URL, get_token};

pub async fn get_auth_apps() -> Result<Vec<AuthApp>> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/auth_apps"].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let auth_apps: Vec<AuthApp> = response.json().await?;
            Ok(auth_apps)
        }
        _ => Err(anyhow!("Failed to fetch auth apps.")),
    }
}

pub async fn get_auth_app(id: &str) -> Result<AuthApp> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/auth_apps/", id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let auth_app: AuthApp = response.json().await?;
            println!("Auth app requested: {auth_app:#?}");
            Ok(auth_app)
        }
        _ => Err(anyhow!(format!(
            "Failed to find an auth app with id: {}.",
            id
        ))),
    }
}

pub async fn delete_auth_app(id: String) -> Result<()> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .delete([BASE_URL, "v1/auth_apps/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("Project deleted.");
            Ok(())
        }
        _ => Err(anyhow!("Failed to delete an auth app.")),
    }
}

pub async fn create_auth_app(auth_app: AuthAppCreate) -> Result<AuthApp> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .post([BASE_URL, "v1/auth_apps"].join(""))
        .json(&auth_app)
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let auth_app: AuthApp = response.json().await?;
            println!("Project created: {auth_app:#?}");
            Ok(auth_app)
        }
        _ => Err(anyhow!("Failed to create an auth app.")),
    }
}
