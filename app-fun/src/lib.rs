pub mod cli;
pub mod handler;

use anyhow::{anyhow, Result};
use log::debug;
use reqwest::Client;
use smbpndk_model::{AppCreate, Fun};
use smbpndk_networking::{constants::BASE_URL, get_token};

async fn get_fun_apps() -> Result<Vec<Fun>> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/fun_apps"].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let fun_apps: Vec<Fun> = response.json().await?;
            Ok(fun_apps)
        }
        _ => Err(anyhow!("Failed to fetch fun apps.")),
    }
}

async fn get_fun_app(id: &str) -> Result<Fun> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .get([BASE_URL, "v1/fun_apps/", id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let fun_app: Fun = response.json().await?;
            println!("Auth app requested: {fun_app:#?}");
            Ok(fun_app)
        }
        _ => Err(anyhow!(format!("Failed to find an fun app with id: {id}."))),
    }
}

async fn delete_fun_app(id: String) -> Result<()> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .delete([BASE_URL, "v1/fun_apps/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("Project deleted.");
            Ok(())
        }
        _ => Err(anyhow!("Failed to delete an fun app.")),
    }
}

async fn create_fun_app(fun_app: AppCreate) -> Result<Fun> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .post([BASE_URL, "v1/fun_apps"].join(""))
        .json(&fun_app)
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let fun_app: Fun = response.json().await?;
            println!("Project created: {fun_app:#?}");
            Ok(fun_app)
        }
        _ => Err(anyhow!("Failed to create an fun app.")),
    }
}
