mod constants;

use anyhow::{anyhow, Result};
use constants::BASE_URL;
use log::debug;
use reqwest::Client;
use smbpndk_model::{AuthApp, AuthAppCreate};

pub async fn get_token() -> Result<String> {
    if let Some(mut path) = dirs::home_dir() {
        path.push(".smb/token");
        std::fs::read_to_string(path).map_err(|e| {
            debug!("Error while reading token: {}", &e);
            anyhow!("Are you logged in?")
        })
    } else {
        Err(anyhow!("Failed to get home directory."))
    }
}

pub async fn get_auth_apps() -> Result<Vec<AuthApp>> {
    // Get current token
    let token = get_token().await.unwrap();

    let response = Client::new()
        .get([BASE_URL, "v1/auth_apps"].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let projects: Vec<AuthApp> = response.json().await?;
            Ok(projects)
        }
        _ => Err(anyhow!("Failed to fetch auth apps.")),
    }
}

pub async fn get_auth_app(id: String) -> Result<AuthApp> {
    // Get current token
    let token = get_token().await.unwrap();

    let response = Client::new()
        .get([BASE_URL, "v1/auth_apps/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let project: AuthApp = response.json().await?;
            println!("Project requested: {project:#?}");
            Ok(project)
        }
        _ => Err(anyhow!("Failed to request an auth app.")),
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
        _ => Err(anyhow!("Failed to delete a project.")),
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
            let project: AuthApp = response.json().await?;
            println!("Project created: {project:#?}");
            Ok(project)
        }
        _ => Err(anyhow!("Failed to create a project.")),
    }
}
