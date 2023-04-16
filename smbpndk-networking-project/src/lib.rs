use anyhow::{anyhow, Result};
use log::debug;
use reqwest::Client;

use smbpndk_model::{Project, ProjectCreate};
use smbpndk_networking::{constants::BASE_URL, get_token};

pub async fn get_all() -> Result<Vec<Project>> {
    // Get current token
    let token = get_token().await?;

    print!("{}", token);

    let response = Client::new()
        .get([BASE_URL, "v1/projects"].join(""))
        .header("Authorization", token)
        .header("User-agent", "smbpndk-cli")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let projects: Vec<Project> = response.json().await?;
            Ok(projects)
        }
        _ => Err(anyhow!("Failed to fetch projects.")),
    }
}

pub async fn create_project(project: ProjectCreate) -> Result<Project> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .post([BASE_URL, "v1/projects"].join(""))
        .json(&project)
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let project: Project = response.json().await?;
            println!("Project created: {project:#?}");
            Ok(project)
        }
        _ => Err(anyhow!("Failed to create a project.")),
    }
}

pub async fn get_project(id: String) -> Result<Project> {
    // Get current token
    let token = get_token().await.unwrap();

    let response = Client::new()
        .get([BASE_URL, "v1/projects/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let project: Project = response.json().await?;
            println!("Project requested: {project:#?}");
            Ok(project)
        }
        _ => Err(anyhow!("Failed to request a project.")),
    }
}

pub async fn delete_project(id: String) -> Result<()> {
    // Get current token
    let token = get_token().await.unwrap();

    let response = Client::new()
        .delete([BASE_URL, "v1/projects/", &id].join(""))
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
